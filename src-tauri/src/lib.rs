mod config;
mod paths;

use tauri::Manager;

/// 已有实例时把主窗口带到前台（第二次启动的进程会直接退出）。
/// Windows 常拦截后台进程的 SetForegroundWindow，用一次置顶脉冲配合 set_focus。
fn focus_main_window(app: &tauri::AppHandle) {
    let Some(window) = app.get_webview_window("main") else {
        return;
    };
    if window.is_minimized().unwrap_or(false) {
        let _ = window.unminimize();
    }
    let _ = window.show();
    let _ = window.set_always_on_top(true);
    let _ = window.set_focus();
    let _ = window.set_always_on_top(false);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let boot_theme = config::load_settings().theme;

    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            focus_main_window(app);
        }))
        .append_invoke_initialization_script(config::theme_boot_script(&boot_theme))
        .invoke_handler(tauri::generate_handler![
            get_settings,
            update_settings,
            save_window_state,
            set_zoom,
            zoom_by,
            uses_custom_titlebar,
        ])
        .setup(|app| {
            if let Some(window) = app.get_webview_window("main") {
                if let Ok(icon) =
                    tauri::image::Image::from_bytes(include_bytes!("../icons/128x128.png"))
                {
                    let _ = window.set_icon(icon);
                }
                let _ = window.set_title(&format!("Lapapp {}", app.package_info().version));

                // 仅 Windows 移除原生装饰，前端渲染自绘标题栏；其余平台保留系统边框
                #[cfg(windows)]
                {
                    let _ = window.set_decorations(false);
                    let _ = window.set_shadow(true);
                }

                config::restore_window_state(&window);
                let settings = config::load_settings();
                config::apply_window_chrome(&window, &settings.theme);
                let _ = window.set_zoom(settings.zoom);

                let handle = app.handle().clone();
                window.on_window_event(move |event| {
                    if matches!(event, tauri::WindowEvent::CloseRequested { .. }) {
                        let _ = config::save_window_state(&handle);
                    }
                });

                let _ = window.show();
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running Lapapp");
}

/// Windows 上使用自绘标题栏，其余平台使用系统装饰。
#[tauri::command]
fn uses_custom_titlebar() -> bool {
    cfg!(windows)
}

#[tauri::command]
fn get_settings() -> config::Settings {
    config::load_settings()
}

#[tauri::command]
fn update_settings(
    app: tauri::AppHandle,
    theme: Option<String>,
    locale: Option<String>,
) -> Result<config::Settings, String> {
    let mut settings = config::load_settings();
    if let Some(theme) = theme {
        if config::THEMES.contains(&theme.as_str()) {
            settings.theme = theme;
        }
    }
    if let Some(locale) = locale {
        let locale = locale.trim().to_string();
        if !locale.is_empty() {
            settings.locale = locale;
        }
    }
    config::save_settings(&settings)?;
    if let Some(window) = app.get_webview_window("main") {
        config::apply_window_chrome(&window, &settings.theme);
    }
    Ok(settings)
}

#[tauri::command]
fn save_window_state(app: tauri::AppHandle) -> Result<(), String> {
    config::save_window_state(&app)
}

#[tauri::command]
fn set_zoom(window: tauri::WebviewWindow, level: f64) -> Result<f64, String> {
    let zoom = level.clamp(config::ZOOM_MIN, config::ZOOM_MAX);
    window.set_zoom(zoom).map_err(|e| e.to_string())?;

    let mut settings = config::load_settings();
    settings.zoom = zoom;
    config::save_settings(&settings)?;
    Ok(zoom)
}

#[tauri::command]
fn zoom_by(window: tauri::WebviewWindow, delta: f64) -> Result<f64, String> {
    let current = config::load_settings().zoom;
    set_zoom(window, current + delta)
}
