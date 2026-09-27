use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use tauri::{AppHandle, Manager, PhysicalPosition, PhysicalSize, WebviewWindow};

use crate::paths;

/// lapstyle 内置主题 ID（tokens.css）
pub const THEMES: [&str; 7] = ["dark", "light", "mint", "sky", "pink", "brown", "amber"];

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    #[serde(default = "default_zoom")]
    pub zoom: f64,
    pub theme: String,
    pub locale: String,
}

pub const ZOOM_MIN: f64 = 0.5;
pub const ZOOM_MAX: f64 = 2.0;

fn default_zoom() -> f64 {
    1.0
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            zoom: default_zoom(),
            theme: "dark".to_string(),
            locale: "zh".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct WindowState {
    pub x: Option<i32>,
    pub y: Option<i32>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    #[serde(default)]
    pub maximized: bool,
}

fn read_json<T: for<'de> Deserialize<'de> + Default>(path: &Path) -> T {
    fs::read_to_string(path)
        .ok()
        .and_then(|text| serde_json::from_str(&text).ok())
        .unwrap_or_default()
}

fn write_json<T: Serialize>(path: &Path, value: &T) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("create {}: {e}", parent.display()))?;
    }
    let json = serde_json::to_string_pretty(value).map_err(|e| e.to_string())?;
    fs::write(path, json).map_err(|e| format!("write {}: {e}", path.display()))
}

pub fn load_settings() -> Settings {
    let mut settings: Settings = read_json(&paths::settings_path());
    if !THEMES.contains(&settings.theme.as_str()) {
        settings.theme = Settings::default().theme;
    }
    if settings.locale.trim().is_empty() {
        settings.locale = Settings::default().locale;
    }
    settings.zoom = settings.zoom.clamp(ZOOM_MIN, ZOOM_MAX);
    settings
}

pub fn save_settings(settings: &Settings) -> Result<(), String> {
    write_json(&paths::settings_path(), settings)
}

/// 在每次页面加载、任何应用 JS 运行之前设置 data-theme，避免主题闪烁。
pub fn theme_boot_script(theme: &str) -> String {
    let id = if THEMES.contains(&theme) { theme } else { "dark" };
    let encoded = serde_json::to_string(id).unwrap_or_else(|_| "\"dark\"".into());
    format!(
        "try{{window.__LAPAPP_THEME__={encoded};document.documentElement.setAttribute(\"data-theme\",{encoded});}}catch(e){{}}"
    )
}

/// 同步原生窗口外观：背景色 + 原生深/浅色（右键菜单、滚动条等）。
pub fn apply_window_chrome(window: &WebviewWindow, theme: &str) {
    // dark 是 lapstyle 唯一的深色主题，其余全部按浅色窗口处理
    let light = theme != "dark";
    let channel = if light { 255 } else { 0x11 };
    let _ = window.set_background_color(Some(tauri::window::Color(
        channel, channel, channel, 255,
    )));
    let _ = window.set_theme(Some(if light {
        tauri::Theme::Light
    } else {
        tauri::Theme::Dark
    }));
}

// ---------------------------------------------------------------------------
// 窗口几何持久化（自 Lapeditor 移植）
// ---------------------------------------------------------------------------

pub fn load_window_state() -> WindowState {
    read_json(&paths::window_state_path())
}

fn is_normal_position(x: i32, y: i32) -> bool {
    // Windows 最小化的窗口会报告 -32000 一类的坐标
    x > -10_000 && y > -10_000 && x < 50_000 && y < 50_000
}

fn is_normal_size(width: u32, height: u32) -> bool {
    // 与 tauri.conf.json 的 minWidth/minHeight 保持一致
    width >= 800 && height >= 500 && width <= 20_000 && height <= 20_000
}

fn decoration_delta(window: &WebviewWindow) -> (u32, u32) {
    let Ok(outer) = window.outer_size() else {
        return (0, 0);
    };
    let Ok(inner) = window.inner_size() else {
        return (0, 0);
    };
    (
        outer.width.saturating_sub(inner.width),
        outer.height.saturating_sub(inner.height),
    )
}

struct RestoredFrame {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
}

/// 最大化时读取 Win32 恢复区（rcNormalPosition），而不是最大化的位置尺寸。
#[cfg(windows)]
fn restored_frame(window: &WebviewWindow) -> Option<RestoredFrame> {
    use windows::Win32::UI::WindowsAndMessaging::{GetWindowPlacement, WINDOWPLACEMENT};

    let hwnd = window.hwnd().ok()?;
    let mut place = WINDOWPLACEMENT::default();
    place.length = std::mem::size_of::<WINDOWPLACEMENT>() as u32;
    unsafe { GetWindowPlacement(hwnd, &mut place).ok()? };

    let rect = place.rcNormalPosition;
    let width = rect.right.saturating_sub(rect.left);
    let height = rect.bottom.saturating_sub(rect.top);
    if width <= 0 || height <= 0 {
        return None;
    }

    Some(RestoredFrame {
        x: rect.left,
        y: rect.top,
        width: width as u32,
        height: height as u32,
    })
}

#[cfg(not(windows))]
fn restored_frame(window: &WebviewWindow) -> Option<RestoredFrame> {
    let position = window.outer_position().ok()?;
    let size = window.inner_size().ok()?;
    Some(RestoredFrame {
        x: position.x,
        y: position.y,
        width: size.width,
        height: size.height,
    })
}

fn save_normal_inner_frame(window: &WebviewWindow, state: &mut WindowState) {
    let Ok(pos) = window.outer_position() else {
        return;
    };
    let Ok(size) = window.inner_size() else {
        return;
    };
    if is_normal_position(pos.x, pos.y) && is_normal_size(size.width, size.height) {
        state.x = Some(pos.x);
        state.y = Some(pos.y);
        state.width = Some(size.width);
        state.height = Some(size.height);
    }
}

/// 把 Win32 外框恢复区换算回 Tauri 的内尺寸。
/// 若直接保存 GetWindowPlacement（外框）再用 set_size 恢复，窗口每次启动都会
/// 被 DWM 阴影/边框撑大一圈。
fn save_inner_frame_from_placement(
    window: &WebviewWindow,
    state: &mut WindowState,
    frame: &RestoredFrame,
) {
    let (pad_w, pad_h) = decoration_delta(window);
    let width = frame.width.saturating_sub(pad_w);
    let height = frame.height.saturating_sub(pad_h);
    if is_normal_position(frame.x, frame.y) && is_normal_size(width, height) {
        state.x = Some(frame.x);
        state.y = Some(frame.y);
        state.width = Some(width);
        state.height = Some(height);
    }
}

pub fn save_window_state(app: &AppHandle) -> Result<(), String> {
    let Some(window) = app.get_webview_window("main") else {
        return Ok(());
    };

    let mut state = load_window_state();
    let minimized = window.is_minimized().unwrap_or(false);
    let maximized = window.is_maximized().unwrap_or(false);

    if !minimized {
        state.maximized = maximized;
    }

    if !minimized && !maximized {
        // 与 restore 配对：Tauri set_size() 是内尺寸，set_position() 是外位置
        save_normal_inner_frame(&window, &mut state);
    } else if !minimized {
        if let Some(frame) = restored_frame(&window) {
            let (pad_w, pad_h) = decoration_delta(&window);
            if pad_w > 0 || pad_h > 0 {
                save_inner_frame_from_placement(&window, &mut state, &frame);
            }
        }
    }

    write_json(&paths::window_state_path(), &state)
}

/// 在窗口显示之前恢复几何信息，避免可见的位置跳动。
pub fn restore_window_state(window: &WebviewWindow) {
    let state = load_window_state();

    if let (Some(width), Some(height)) = (state.width, state.height) {
        if is_normal_size(width, height) {
            let _ = window.set_size(PhysicalSize::new(width, height));
        }
    }
    if let (Some(x), Some(y)) = (state.x, state.y) {
        if is_normal_position(x, y) {
            let _ = window.set_position(PhysicalPosition::new(x, y));
        }
    }
    if state.maximized {
        let _ = window.maximize();
    }
}
