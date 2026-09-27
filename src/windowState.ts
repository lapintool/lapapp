import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";

/// 窗口移动/缩放停止 1 秒后保存几何信息（关闭时 Rust 端还会兜底保存一次）。
export function bindWindowStateSave(): void {
  let timer: number | undefined;

  const schedule = (): void => {
    if (timer !== undefined) clearTimeout(timer);
    timer = window.setTimeout(() => {
      timer = undefined;
      void invoke("save_window_state").catch((err) => {
        console.warn("failed to save window state", err);
      });
    }, 1000);
  };

  try {
    const win = getCurrentWindow();
    void win.onMoved(schedule);
    void win.onResized(schedule);
  } catch {
    /* 非 Tauri 环境（纯浏览器调试）忽略 */
  }
}
