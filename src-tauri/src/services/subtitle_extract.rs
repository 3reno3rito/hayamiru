use std::fmt::Write;
use std::path::Path;
use std::process::Command;

/// Read a text file, handling BOM and non-UTF-8 encodings (Latin-1 fallback).
fn read_text_file(path: &str) -> Result<String, String> {
    let bytes = std::fs::read(path).map_err(|e| format!("Cannot read: {e}"))?;
    let bytes = if bytes.starts_with(&[0xEF, 0xBB, 0xBF]) { &bytes[3..] } else { &bytes };
    match std::str::from_utf8(bytes) {
        Ok(s) => Ok(s.to_string()),
        Err(_) => Ok(bytes.iter().map(|&b| b as char).collect()),
    }
}

#[derive(Clone)]
pub struct SubEntry {
    pub start_ms: u64,
    pub end_ms: u64,
    pub text: String,
}

/// Extract subtitles from MKV/MP4 using ffprobe (exact timing).
pub fn extract_from_video(path: &str, track_index: Option<i64>) -> Result<Vec<SubEntry>, String> {
    let ffprobe = find_ffprobe().ok_or("ffprobe not found")?;

    // Find the subtitle stream index
    let stream_idx = track_index
        .map(|i| format!("0:s:{}", i.saturating_sub(1)))
        .unwrap_or_else(|| "0:s:0".into());

    let output = Command::new(&ffprobe)
        .args([
            "-v", "quiet",
            "-select_streams", &stream_idx,
            "-show_entries", "packet=pts_time,duration_time,data",
            "-of", "csv=p=0",
            path,
        ])
        .output()
        .map_err(|e| format!("ffprobe failed: {e}"))?;

    if !output.status.success() {
        // Fallback: try extracting as SRT via ffmpeg
        return extract_via_ffmpeg(path, &stream_idx);
    }

    let text = String::from_utf8_lossy(&output.stdout);
    let mut entries = Vec::new();

    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() { continue; }
        // Format: pts_time,duration_time,data
        // But subtitle data often has commas, so we parse carefully
        let parts: Vec<&str> = line.splitn(3, ',').collect();
        if parts.len() < 3 { continue; }

        let start_s: f64 = parts[0].parse().unwrap_or(0.0);
        let dur_s: f64 = parts[1].parse().unwrap_or(3.0);
        let text = strip_ass_if_needed(parts[2].trim());

        if text.is_empty() { continue; }

        entries.push(SubEntry {
            start_ms: (start_s * 1000.0) as u64,
            end_ms: ((start_s + dur_s) * 1000.0) as u64,
            text,
        });
    }

    if entries.is_empty() {
        // ffprobe packet mode didn't work, try ffmpeg extraction
        return extract_via_ffmpeg(path, &stream_idx);
    }

    Ok(entries)
}

/// Fallback: extract subtitles via ffmpeg to SRT format
fn extract_via_ffmpeg(path: &str, stream: &str) -> Result<Vec<SubEntry>, String> {
    let ffmpeg = find_ffmpeg().ok_or("ffmpeg not found")?;

    let output = Command::new(&ffmpeg)
        .args(["-i", path, "-map", stream, "-f", "srt", "-"])
        .output()
        .map_err(|e| format!("ffmpeg failed: {e}"))?;

    let text = String::from_utf8_lossy(&output.stdout);
    if text.trim().is_empty() {
        return Err("No subtitle data extracted".into());
    }

    parse_srt_content(&text.replace("\r\n", "\n"))
}

pub fn extract_from_srt(path: &str) -> Result<Vec<SubEntry>, String> {
    let content = read_text_file(path)?.replace("\r\n", "\n");
    parse_srt_content(&content)
}

fn parse_srt_content(content: &str) -> Result<Vec<SubEntry>, String> {
    let mut entries = Vec::new();
    for block in content.split("\n\n") {
        let lines: Vec<&str> = block.trim().lines().collect();
        if lines.len() < 3 { continue; }
        let Some((start, end)) = parse_srt_timing(lines[1]) else { continue };
        let text = lines[2..].join("\n").trim().to_string();
        if !text.is_empty() { entries.push(SubEntry { start_ms: start, end_ms: end, text }); }
    }
    Ok(entries)
}

pub fn extract_from_ass(path: &str) -> Result<Vec<SubEntry>, String> {
    let content = read_text_file(path)?.replace("\r\n", "\n");
    let mut entries = Vec::new();
    for line in content.lines() {
        if !line.starts_with("Dialogue:") { continue; }
        let data = &line["Dialogue:".len()..].trim_start();
        let parts: Vec<&str> = data.splitn(10, ',').collect();
        if parts.len() < 10 { continue; }
        let start = ass_time_to_ms(parts[1].trim());
        let end = ass_time_to_ms(parts[2].trim());
        let text = strip_ass(parts[9].trim());
        if !text.is_empty() { entries.push(SubEntry { start_ms: start, end_ms: end, text }); }
    }
    entries.sort_by_key(|e| e.start_ms);
    Ok(entries)
}

pub fn write_srt(entries: &[SubEntry], path: &str) -> Result<(), String> {
    let mut srt = String::with_capacity(entries.len() * 80);
    for (i, e) in entries.iter().enumerate() {
        let _ = writeln!(srt, "{}", i + 1);
        let _ = writeln!(srt, "{} --> {}", ms_to_srt(e.start_ms), ms_to_srt(e.end_ms));
        let _ = writeln!(srt, "{}", e.text);
        let _ = writeln!(srt);
    }
    std::fs::write(path, srt).map_err(|e| format!("Cannot write: {e}"))
}

// --- Helpers ---

fn strip_ass_if_needed(text: &str) -> String {
    if text.contains('{') && text.contains('}') {
        // Might be ASS data with metadata prefix
        let parts: Vec<&str> = text.splitn(9, ',').collect();
        if parts.len() >= 9 {
            return strip_ass(parts[8].trim());
        }
    }
    strip_ass(text)
}

fn strip_ass(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut in_tag = false;
    for c in text.chars() {
        match c { '{' => in_tag = true, '}' => in_tag = false, _ if !in_tag => out.push(c), _ => {} }
    }
    out.replace("\\N", "\n").replace("\\n", "\n")
}

fn parse_srt_timing(line: &str) -> Option<(u64, u64)> {
    let parts: Vec<&str> = line.split("-->").collect();
    if parts.len() != 2 { return None; }
    Some((srt_time_to_ms(parts[0].trim())?, srt_time_to_ms(parts[1].trim())?))
}

fn srt_time_to_ms(t: &str) -> Option<u64> {
    let t = t.replace(',', ".");
    let p: Vec<&str> = t.split(':').collect();
    if p.len() != 3 { return None; }
    let h: u64 = p[0].parse().ok()?;
    let m: u64 = p[1].parse().ok()?;
    let sp: Vec<&str> = p[2].split('.').collect();
    let s: u64 = sp[0].parse().ok()?;
    let ms: u64 = sp.get(1).and_then(|f| format!("{:0<3}", &f[..f.len().min(3)]).parse().ok()).unwrap_or(0);
    Some(h * 3600000 + m * 60000 + s * 1000 + ms)
}

fn ass_time_to_ms(t: &str) -> u64 {
    let p: Vec<&str> = t.split(':').collect();
    if p.len() != 3 { return 0; }
    let h: u64 = p[0].parse().unwrap_or(0);
    let m: u64 = p[1].parse().unwrap_or(0);
    let sp: Vec<&str> = p[2].split('.').collect();
    let s: u64 = sp[0].parse().unwrap_or(0);
    let cs: u64 = sp.get(1).and_then(|v| v.parse().ok()).unwrap_or(0);
    h * 3600000 + m * 60000 + s * 1000 + cs * 10
}

fn ms_to_srt(ms: u64) -> String {
    format!("{:02}:{:02}:{:02},{:03}", ms / 3600000, (ms % 3600000) / 60000, (ms % 60000) / 1000, ms % 1000)
}

fn find_ffprobe() -> Option<String> {
    // Check PATH first
    if Command::new("ffprobe").arg("-version").output().is_ok() { return Some("ffprobe".into()); }
    // Common locations
    for p in [
        "C:/Users/mochi/Downloads/ffmpeg-8.1-essentials_build/bin/ffprobe.exe",
        "C:/ffmpeg/bin/ffprobe.exe",
        "C:/Program Files/ffmpeg/bin/ffprobe.exe",
    ] {
        if Path::new(p).exists() { return Some(p.into()); }
    }
    None
}

fn find_ffmpeg() -> Option<String> {
    if Command::new("ffmpeg").arg("-version").output().is_ok() { return Some("ffmpeg".into()); }
    for p in [
        "C:/Users/mochi/Downloads/ffmpeg-8.1-essentials_build/bin/ffmpeg.exe",
        "C:/ffmpeg/bin/ffmpeg.exe",
        "C:/Program Files/ffmpeg/bin/ffmpeg.exe",
    ] {
        if Path::new(p).exists() { return Some(p.into()); }
    }
    None
}
