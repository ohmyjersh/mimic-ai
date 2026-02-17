use serde::Serialize;
use std::sync::Mutex;
use std::time::Instant;

const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");
const CARGO_TOML_URL: &str = "https://raw.githubusercontent.com/ohmyjersh/mimic-ai/main/Cargo.toml";
const REQUEST_TIMEOUT_SECS: u64 = 5;
const DEFAULT_TTL_SECS: u64 = 3600;

#[derive(Debug, Clone, Serialize)]
pub struct VersionInfo {
    pub current: String,
    pub latest: Option<String>,
    pub update_available: bool,
}

struct CachedVersion {
    info: VersionInfo,
    fetched_at: Instant,
}

pub struct VersionChecker {
    cache: Mutex<Option<CachedVersion>>,
    ttl: u64,
}

impl Default for VersionChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl VersionChecker {
    pub fn new() -> Self {
        Self {
            cache: Mutex::new(None),
            ttl: DEFAULT_TTL_SECS,
        }
    }

    #[cfg(test)]
    fn with_ttl(ttl: u64) -> Self {
        Self {
            cache: Mutex::new(None),
            ttl,
        }
    }

    pub fn cached(&self) -> Option<VersionInfo> {
        let guard = self.cache.lock().unwrap();
        guard.as_ref().and_then(|c| {
            if c.fetched_at.elapsed().as_secs() < self.ttl {
                Some(c.info.clone())
            } else {
                None
            }
        })
    }

    pub async fn check(&self) -> VersionInfo {
        if let Some(info) = self.cached() {
            return info;
        }

        let info = match fetch_latest_version().await {
            Ok(latest) => {
                let update_available = is_newer(&latest, CURRENT_VERSION);
                VersionInfo {
                    current: CURRENT_VERSION.to_string(),
                    latest: Some(latest),
                    update_available,
                }
            }
            Err(e) => {
                eprintln!("mimic: failed to check for updates: {}", e);
                VersionInfo {
                    current: CURRENT_VERSION.to_string(),
                    latest: None,
                    update_available: false,
                }
            }
        };

        let mut guard = self.cache.lock().unwrap();
        *guard = Some(CachedVersion {
            info: info.clone(),
            fetched_at: Instant::now(),
        });

        info
    }
}

async fn fetch_latest_version() -> Result<String, String> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(REQUEST_TIMEOUT_SECS))
        .build()
        .map_err(|e| format!("failed to build HTTP client: {}", e))?;

    let body = client
        .get(CARGO_TOML_URL)
        .send()
        .await
        .map_err(|e| format!("failed to fetch Cargo.toml: {}", e))?
        .text()
        .await
        .map_err(|e| format!("failed to read response body: {}", e))?;

    parse_version_from_cargo_toml(&body)
}

pub fn parse_version_from_cargo_toml(content: &str) -> Result<String, String> {
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("version") {
            if let Some((_key, value)) = trimmed.split_once('=') {
                let version = value.trim().trim_matches('"');
                if version.split('.').count() == 3 {
                    return Ok(version.to_string());
                }
                return Err(format!("malformed version string: {}", version));
            }
        }
    }
    Err("version field not found in Cargo.toml".to_string())
}

pub fn is_newer(latest: &str, current: &str) -> bool {
    let parse = |v: &str| -> Option<(u64, u64, u64)> {
        let parts: Vec<&str> = v.split('.').collect();
        if parts.len() != 3 {
            return None;
        }
        Some((
            parts[0].parse().ok()?,
            parts[1].parse().ok()?,
            parts[2].parse().ok()?,
        ))
    };

    match (parse(latest), parse(current)) {
        (Some(l), Some(c)) => l > c,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_version_valid() {
        let content = "[package]\nname = \"mimic-ai\"\nversion = \"1.2.3\"\nedition = \"2021\"\n";
        assert_eq!(parse_version_from_cargo_toml(content).unwrap(), "1.2.3");
    }

    #[test]
    fn parse_version_missing() {
        let content = "[package]\nname = \"mimic-ai\"\nedition = \"2021\"\n";
        let err = parse_version_from_cargo_toml(content).unwrap_err();
        assert!(err.contains("not found"));
    }

    #[test]
    fn parse_version_malformed() {
        let content = "[package]\nversion = \"bad\"\n";
        let err = parse_version_from_cargo_toml(content).unwrap_err();
        assert!(err.contains("malformed"));
    }

    #[test]
    fn is_newer_patch() {
        assert!(is_newer("1.0.1", "1.0.0"));
    }

    #[test]
    fn is_newer_minor() {
        assert!(is_newer("1.1.0", "1.0.9"));
    }

    #[test]
    fn is_newer_major() {
        assert!(is_newer("2.0.0", "1.9.9"));
    }

    #[test]
    fn is_newer_same() {
        assert!(!is_newer("1.0.0", "1.0.0"));
    }

    #[test]
    fn is_newer_older() {
        assert!(!is_newer("0.9.0", "1.0.0"));
    }

    #[test]
    fn is_newer_invalid() {
        assert!(!is_newer("bad", "1.0.0"));
        assert!(!is_newer("1.0.0", "bad"));
        assert!(!is_newer("bad", "bad"));
    }

    #[test]
    fn cached_returns_none_when_empty() {
        let checker = VersionChecker::new();
        assert!(checker.cached().is_none());
    }

    #[tokio::test]
    async fn cached_returns_value_after_population() {
        let checker = VersionChecker::with_ttl(3600);
        let info = checker.check().await;
        assert_eq!(info.current, CURRENT_VERSION);
        // After check(), cached should return Some
        let cached = checker.cached();
        assert!(cached.is_some());
        assert_eq!(cached.unwrap().current, CURRENT_VERSION);
    }
}
