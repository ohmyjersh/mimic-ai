use clap::Parser;
use mimic_ai::cli::{Cli, Commands};
use mimic_ai::lint;
use mimic_ai::registry::Registry;
use mimic_ai::server::MimicServer;
use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use std::time::Instant;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Lint { warnings }) => {
            std::process::exit(lint::run(warnings));
        }
        None => {
            run_server().await?;
        }
    }

    Ok(())
}

async fn run_server() -> Result<(), Box<dyn std::error::Error>> {
    let project_dir = find_project_dir();
    let registry = Arc::new(RwLock::new(Registry::new(project_dir.clone())));

    let watcher = setup_watcher(Arc::clone(&registry), project_dir);
    let handler = MimicServer::new(registry, watcher);

    let (stdin, stdout) = rmcp::transport::stdio();
    let service = rmcp::serve_server(handler, (stdin, stdout)).await?;
    service.waiting().await?;

    Ok(())
}

fn setup_watcher(
    registry: Arc<RwLock<Registry>>,
    project_dir: Option<PathBuf>,
) -> Option<RecommendedWatcher> {
    let watched_dirs = {
        let reg = registry.read().unwrap();
        reg.watched_dirs()
    };

    if watched_dirs.is_empty() {
        return None;
    }

    let last_rebuild = Arc::new(std::sync::Mutex::new(Instant::now()));
    let rebuild_project_dir = project_dir;

    let mut watcher = match notify::recommended_watcher(move |res: Result<Event, notify::Error>| {
        if res.is_err() {
            return;
        }

        // Debounce: ignore events within 500ms of last rebuild
        let mut last = last_rebuild.lock().unwrap();
        if last.elapsed().as_millis() < 500 {
            return;
        }
        *last = Instant::now();
        drop(last);

        let new_registry = Registry::new(rebuild_project_dir.clone());
        if let Ok(mut reg) = registry.write() {
            *reg = new_registry;
        }
        eprintln!("mimic: registry reloaded");
    }) {
        Ok(w) => w,
        Err(e) => {
            eprintln!("mimic: warning: failed to set up file watcher: {}", e);
            return None;
        }
    };

    for dir in &watched_dirs {
        if let Err(e) = watcher.watch(dir, RecursiveMode::Recursive) {
            eprintln!("mimic: warning: failed to watch {}: {}", dir.display(), e);
        }
    }

    Some(watcher)
}

fn find_project_dir() -> Option<PathBuf> {
    let cwd = std::env::current_dir().ok()?;
    find_project_dir_from(&cwd)
}

fn find_project_dir_from(start: &std::path::Path) -> Option<PathBuf> {
    let mut dir = start;
    loop {
        let candidate = dir.join(".mimic");
        if candidate.is_dir() {
            return Some(candidate);
        }
        dir = dir.parent()?;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_project_dir_in_current() {
        let tmp = tempfile::tempdir().unwrap();
        let mimic = tmp.path().join(".mimic");
        std::fs::create_dir(&mimic).unwrap();
        let result = find_project_dir_from(tmp.path());
        assert_eq!(result, Some(mimic));
    }

    #[test]
    fn find_project_dir_in_parent() {
        let tmp = tempfile::tempdir().unwrap();
        let mimic = tmp.path().join(".mimic");
        std::fs::create_dir(&mimic).unwrap();
        let child = tmp.path().join("sub").join("deep");
        std::fs::create_dir_all(&child).unwrap();
        let result = find_project_dir_from(&child);
        assert_eq!(result, Some(mimic));
    }

    #[test]
    fn find_project_dir_not_found() {
        let tmp = tempfile::tempdir().unwrap();
        let child = tmp.path().join("empty");
        std::fs::create_dir_all(&child).unwrap();
        let result = find_project_dir_from(&child);
        assert!(result.is_none());
    }

    #[test]
    fn find_project_dir_file_not_dir() {
        let tmp = tempfile::tempdir().unwrap();
        let mimic = tmp.path().join(".mimic");
        std::fs::write(&mimic, "not a directory").unwrap();
        let result = find_project_dir_from(tmp.path());
        assert!(result.is_none());
    }
}
