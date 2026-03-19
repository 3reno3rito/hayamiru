use std::io::{Read, Seek, SeekFrom};
use std::sync::Arc;
use tokio::sync::Mutex;
use reqwest::Client;
use serde::{Deserialize, Serialize};

const API_URL: &str = "https://api.opensubtitles.com/api/v1";
const USER_AGENT: &str = "Hayamiru v0.1.0";

/// Shared client that caches the reqwest::Client and auth token across calls.
pub struct OsClient {
    client: Client,
    api_key: String,
    token: Option<String>,
    username: String,
    password: String,
}

#[derive(Serialize)]
struct LoginBody { username: String, password: String }

#[derive(Deserialize)]
struct LoginResponse { token: String }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubResult {
    pub name: String,
    pub lang: String,
    pub download_count: i64,
    pub rating: f64,
    pub file_id: i64,
}

#[derive(Deserialize)]
struct SearchResponse { data: Vec<SearchItem> }

#[derive(Deserialize)]
struct SearchItem { attributes: SearchAttr }

#[derive(Deserialize)]
struct SearchAttr {
    language: String,
    download_count: i64,
    ratings: f64,
    files: Vec<SearchFile>,
}

#[derive(Deserialize)]
struct SearchFile { file_id: i64, file_name: String }

#[derive(Deserialize)]
struct DownloadResponse { link: String }

/// Global shared instance — avoids re-login and re-creating HTTP client.
static INSTANCE: std::sync::OnceLock<Arc<Mutex<Option<OsClient>>>> = std::sync::OnceLock::new();

fn global() -> &'static Arc<Mutex<Option<OsClient>>> {
    INSTANCE.get_or_init(|| Arc::new(Mutex::new(None)))
}

/// Get or create the shared client. Re-creates if credentials changed.
pub async fn get_client(api_key: &str, username: &str, password: &str) -> Result<(), String> {
    let mut guard = global().lock().await;
    if let Some(c) = guard.as_ref() {
        if c.api_key == api_key && c.username == username { return Ok(()); }
    }
    let mut client = OsClient {
        client: Client::new(),
        api_key: api_key.to_string(),
        token: None,
        username: username.to_string(),
        password: password.to_string(),
    };
    if !username.is_empty() && !password.is_empty() {
        client.login().await?;
    }
    *guard = Some(client);
    Ok(())
}

impl OsClient {
    async fn login(&mut self) -> Result<(), String> {
        let resp = self.client.post(format!("{API_URL}/login"))
            .header("Api-Key", &self.api_key)
            .header("User-Agent", USER_AGENT)
            .header("Content-Type", "application/json")
            .json(&LoginBody { username: self.username.clone(), password: self.password.clone() })
            .send().await.map_err(|e| format!("Login failed: {e}"))?;

        if !resp.status().is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(format!("Login failed: {body}"));
        }
        let body: LoginResponse = resp.json().await.map_err(|e| format!("Parse error: {e}"))?;
        self.token = Some(body.token);
        Ok(())
    }
}

/// Search by hash first, fallback to filename if insufficient results.
pub async fn search(hash: &str, filename: &str, lang: &str) -> Result<Vec<SubResult>, String> {
    let guard = global().lock().await;
    let client = guard.as_ref().ok_or("Not initialized")?;

    // Step 1: search by hash (most accurate)
    let mut results = Vec::new();
    if !hash.is_empty() {
        results = do_search(&client.client, &client.api_key, &format!(
            "{}moviehash={hash}", lang_param(lang)
        )).await?;
    }

    // Step 2: if few results, search by filename
    if results.len() < 5 && !filename.is_empty() {
        let name_results = do_search(&client.client, &client.api_key, &format!(
            "{}query={}", lang_param(lang), urlencoding::encode(filename)
        )).await?;
        // Merge, dedup by file_id
        for r in name_results {
            if !results.iter().any(|existing| existing.file_id == r.file_id) {
                results.push(r);
            }
        }
    }

    // Sort: most downloaded first
    results.sort_by(|a, b| b.download_count.cmp(&a.download_count));
    Ok(results)
}

fn lang_param(lang: &str) -> String {
    if lang.is_empty() { String::new() } else { format!("languages={lang}&") }
}

async fn do_search(client: &Client, api_key: &str, params: &str) -> Result<Vec<SubResult>, String> {
    let url = format!("{API_URL}/subtitles?{params}");
    let resp = client.get(&url)
        .header("Api-Key", api_key)
        .header("User-Agent", USER_AGENT)
        .send().await.map_err(|e| format!("Search failed: {e}"))?;

    if !resp.status().is_success() {
        let body = resp.text().await.unwrap_or_default();
        return Err(format!("Search failed: {body}"));
    }

    let body: SearchResponse = resp.json().await.map_err(|e| format!("Parse error: {e}"))?;
    Ok(body.data.into_iter().flat_map(|item| {
        let a = item.attributes;
        a.files.into_iter().map(move |f| SubResult {
            name: f.file_name.clone(),
            lang: a.language.clone(),
            download_count: a.download_count,
            rating: a.ratings,
            file_id: f.file_id,
        })
    }).collect())
}

pub async fn download(file_id: i64) -> Result<String, String> {
    let mut guard = global().lock().await;
    let client = guard.as_mut().ok_or("Not initialized")?;

    // Re-login if token missing
    if client.token.is_none() {
        client.login().await?;
    }
    let token = client.token.as_ref().ok_or("Not logged in")?;

    let resp = client.client.post(format!("{API_URL}/download"))
        .header("Api-Key", &client.api_key)
        .header("User-Agent", USER_AGENT)
        .header("Authorization", format!("Bearer {token}"))
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({ "file_id": file_id }))
        .send().await.map_err(|e| format!("Download failed: {e}"))?;

    if !resp.status().is_success() {
        let body = resp.text().await.unwrap_or_default();
        // Token expired — re-login and retry once
        if body.contains("401") || body.contains("Unauthorized") {
            drop(guard);
            let mut guard = global().lock().await;
            let client = guard.as_mut().ok_or("Not initialized")?;
            client.login().await?;
            let token = client.token.as_ref().ok_or("Not logged in")?;
            let resp = client.client.post(format!("{API_URL}/download"))
                .header("Api-Key", &client.api_key)
                .header("User-Agent", USER_AGENT)
                .header("Authorization", format!("Bearer {token}"))
                .header("Content-Type", "application/json")
                .json(&serde_json::json!({ "file_id": file_id }))
                .send().await.map_err(|e| format!("Retry failed: {e}"))?;
            if !resp.status().is_success() {
                let body = resp.text().await.unwrap_or_default();
                return Err(format!("Download failed: {body}"));
            }
            let dl: DownloadResponse = resp.json().await.map_err(|e| format!("Parse error: {e}"))?;
            return client.client.get(&dl.link).send().await
                .map_err(|e| format!("Fetch failed: {e}"))?
                .text().await.map_err(|e| format!("Read failed: {e}"));
        }
        return Err(format!("Download failed: {body}"));
    }

    let dl: DownloadResponse = resp.json().await.map_err(|e| format!("Parse error: {e}"))?;
    client.client.get(&dl.link).send().await
        .map_err(|e| format!("Fetch failed: {e}"))?
        .text().await.map_err(|e| format!("Read failed: {e}"))
}

/// Compute OpenSubtitles hash. Reads 64KB from start + end in 2 syscalls.
pub fn compute_hash(path: &str) -> Result<String, String> {
    let mut file = std::fs::File::open(path).map_err(|e| format!("Cannot open: {e}"))?;
    let size = file.metadata().map_err(|e| format!("Cannot read size: {e}"))?.len();
    if size < 131072 { return Err("File too small".into()); }

    let mut hash: u64 = size;
    let mut buf = [0u8; 65536];

    // First 64KB — one read
    file.read_exact(&mut buf).map_err(|e| format!("Read error: {e}"))?;
    for chunk in buf.chunks_exact(8) {
        hash = hash.wrapping_add(u64::from_le_bytes(chunk.try_into().unwrap()));
    }

    // Last 64KB — one read
    file.seek(SeekFrom::End(-65536)).map_err(|e| format!("Seek error: {e}"))?;
    file.read_exact(&mut buf).map_err(|e| format!("Read error: {e}"))?;
    for chunk in buf.chunks_exact(8) {
        hash = hash.wrapping_add(u64::from_le_bytes(chunk.try_into().unwrap()));
    }

    Ok(format!("{:016x}", hash))
}
