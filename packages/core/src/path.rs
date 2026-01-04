use std::path::PathBuf;

pub fn get_exe_dir() -> PathBuf {
    std::env::current_exe()
        .ok()
        .and_then(|mut path| {
            path.pop();
            Some(path)
        })
        .unwrap_or_else(|| PathBuf::from("."))
}

pub fn get_exe_relative_path(relative_path: &str) -> PathBuf {
    get_exe_dir().join(relative_path)
}
