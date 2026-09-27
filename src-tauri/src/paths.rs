use std::path::{Path, PathBuf};

/// 可移植布局：优先 LAPAPP_ROOT 环境变量；开发期写入仓库根目录；
/// 发布后使用 exe 所在目录。
pub fn app_root() -> PathBuf {
    if let Ok(dir) = std::env::var("LAPAPP_ROOT") {
        return PathBuf::from(dir);
    }
    #[cfg(debug_assertions)]
    {
        return Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    }
    #[cfg(not(debug_assertions))]
    {
        std::env::current_exe()
            .ok()
            .and_then(|exe| exe.parent().map(Path::to_path_buf))
            .unwrap_or_else(|| PathBuf::from("."))
    }
}

pub fn config_dir() -> PathBuf {
    app_root().join("config")
}

pub fn window_state_path() -> PathBuf {
    config_dir().join("window.json")
}

pub fn settings_path() -> PathBuf {
    config_dir().join("settings.json")
}
