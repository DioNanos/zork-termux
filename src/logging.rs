use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};

struct LoggerState {
    path: PathBuf,
    file: File,
}

static LOGGER: OnceLock<Mutex<LoggerState>> = OnceLock::new();

pub fn init() -> Option<PathBuf> {
    if !is_enabled() {
        return None;
    }

    if let Some(path) = log_path() {
        return Some(path);
    }

    let dir = log_dir();
    fs::create_dir_all(&dir).ok()?;
    cleanup_old_logs(&dir, 30);

    let path = dir.join(format!("session-{}.log", unix_ts()));
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .ok()?;

    let state = LoggerState {
        path: path.clone(),
        file,
    };

    if LOGGER.set(Mutex::new(state)).is_err() {
        return log_path();
    }

    install_panic_hook();
    info(format!(
        "session.start pid={} cwd={}",
        std::process::id(),
        cwd()
    ));

    Some(path)
}

pub fn log_path() -> Option<PathBuf> {
    LOGGER
        .get()
        .and_then(|m| m.lock().ok().map(|state| state.path.clone()))
}

pub fn info<M: AsRef<str>>(message: M) {
    write("INFO", message.as_ref());
}

pub fn warn<M: AsRef<str>>(message: M) {
    write("WARN", message.as_ref());
}

pub fn error<M: AsRef<str>>(message: M) {
    write("ERROR", message.as_ref());
}

fn write(level: &str, message: &str) {
    let Some(mutex) = LOGGER.get() else {
        return;
    };

    let Ok(mut state) = mutex.lock() else {
        return;
    };

    let clean = message.replace('\n', "\\n");
    let _ = writeln!(state.file, "{} [{}] {}", unix_ts(), level, clean);
    let _ = state.file.flush();
}

fn is_enabled() -> bool {
    match std::env::var("ZORK_LOG") {
        Ok(value) => {
            let v = value.trim().to_ascii_lowercase();
            !(v == "0" || v == "false" || v == "off" || v == "no")
        }
        Err(_) => true,
    }
}

fn log_dir() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(".zork-termux").join("logs")
}

fn unix_ts() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

fn cwd() -> String {
    std::env::current_dir()
        .ok()
        .unwrap_or_else(|| PathBuf::from("."))
        .display()
        .to_string()
}

fn cleanup_old_logs(dir: &Path, keep: usize) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };

    let mut files: Vec<(std::time::SystemTime, PathBuf)> = entries
        .flatten()
        .filter_map(|entry| {
            let path = entry.path();
            let name = path.file_name()?.to_str()?;
            if !name.starts_with("session-") || !name.ends_with(".log") {
                return None;
            }
            let modified = entry
                .metadata()
                .ok()
                .and_then(|m| m.modified().ok())
                .unwrap_or(UNIX_EPOCH);
            Some((modified, path))
        })
        .collect();

    files.sort_by_key(|(modified, _)| *modified);

    while files.len() > keep {
        if let Some((_, path)) = files.first() {
            let _ = fs::remove_file(path);
        }
        files.remove(0);
    }
}

fn install_panic_hook() {
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let location = panic_info
            .location()
            .map(|l| format!("{}:{}", l.file(), l.line()))
            .unwrap_or_else(|| "unknown".to_string());

        let payload = if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            (*s).to_string()
        } else if let Some(s) = panic_info.payload().downcast_ref::<String>() {
            s.clone()
        } else {
            "non-string panic payload".to_string()
        };

        error(format!("panic location={} payload={}", location, payload));
        default_hook(panic_info);
    }));
}
