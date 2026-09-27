import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { applyLocale } from "./i18n";

export interface AppSettings {
  theme: string;
  locale: string;
}

/// lapstyle 内置主题（与 src-tauri config::THEMES 保持一致）
export const THEMES = [
  "dark",
  "light",
  "mint",
  "sky",
  "pink",
  "brown",
  "amber",
] as const;
export type ThemeId = (typeof THEMES)[number];

export const theme = ref<ThemeId>("dark");
export const locale = ref<string>("zh");

function isThemeId(value: string): value is ThemeId {
  return (THEMES as readonly string[]).includes(value);
}

export function applyTheme(id: ThemeId): void {
  document.documentElement.dataset.theme = id;
  try {
    localStorage.setItem("lapapp-theme", id);
  } catch {
    /* ignore */
  }
}

/// 启动时引导外观：先用 localStorage 缓存抗闪烁，再以 Rust settings.json 为准。
export async function initSettings(): Promise<void> {
  try {
    const cached = localStorage.getItem("lapapp-theme");
    if (cached && isThemeId(cached)) applyTheme(cached);
  } catch {
    /* ignore */
  }
  try {
    const settings = await invoke<AppSettings>("get_settings");
    if (isThemeId(settings.theme)) {
      theme.value = settings.theme;
      applyTheme(settings.theme);
    }
    locale.value = settings.locale;
    applyLocale(settings.locale);
  } catch (err) {
    console.warn("failed to load settings", err);
  }
}

export async function setTheme(id: ThemeId): Promise<void> {
  theme.value = id;
  applyTheme(id);
  try {
    await invoke<AppSettings>("update_settings", { theme: id });
  } catch (err) {
    console.warn("failed to save theme", err);
  }
}

export async function setLocale(id: string): Promise<void> {
  if (id !== "zh" && id !== "en") return;
  locale.value = id;
  applyLocale(id);
  try {
    await invoke<AppSettings>("update_settings", { locale: id });
  } catch (err) {
    console.warn("failed to save locale", err);
  }
}
