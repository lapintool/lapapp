import { invoke } from "@tauri-apps/api/core";

/// 与常见浏览器页面缩放步长一致
const ZOOM_STEP = 0.1;

type ZoomAction = "in" | "out" | "reset";

/// Ctrl/Cmd + / - / 0（含小键盘），与 Chromium 页面缩放一致
function zoomAction(e: KeyboardEvent): ZoomAction | null {
  if (!(e.ctrlKey || e.metaKey) || e.altKey) {
    return null;
  }
  const key = e.key;
  const code = e.code;
  if (key === "0" || code === "Digit0" || code === "Numpad0") {
    return "reset";
  }
  if (key === "=" || key === "+" || code === "Equal" || code === "NumpadAdd") {
    return "in";
  }
  if (key === "-" || key === "_" || code === "Minus" || code === "NumpadSubtract") {
    return "out";
  }
  return null;
}

/// Ctrl + 加减/0 与 Ctrl + 滚轮缩放界面，范围 0.5–2.0，持久化到 settings.json。
export function bindZoomShortcuts(): void {
  document.addEventListener(
    "keydown",
    (e) => {
      const action = zoomAction(e);
      if (!action) {
        return;
      }
      e.preventDefault();
      void (async () => {
        try {
          if (action === "reset") {
            await invoke("set_zoom", { level: 1 });
          } else {
            await invoke("zoom_by", {
              delta: action === "in" ? ZOOM_STEP : -ZOOM_STEP,
            });
          }
        } catch (err) {
          console.error("zoom hotkey failed:", err);
        }
      })();
    },
    { capture: true },
  );

  document.addEventListener(
    "wheel",
    (e) => {
      if (!(e.ctrlKey || e.metaKey)) {
        return;
      }
      e.preventDefault();
      const delta = e.deltaY < 0 ? ZOOM_STEP : -ZOOM_STEP;
      void invoke("zoom_by", { delta }).catch((err) => {
        console.error("zoom wheel failed:", err);
      });
    },
    { passive: false, capture: true },
  );
}
