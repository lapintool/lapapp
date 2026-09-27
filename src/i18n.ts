import { createI18n } from "vue-i18n";
import en from "./locales/en";
import zh from "./locales/zh";

export type LocaleId = "zh" | "en";

/// 语言清单（label 保持各自母语，不参与翻译）
export const LOCALES: { id: LocaleId; label: string }[] = [
  { id: "zh", label: "简体中文" },
  { id: "en", label: "English" },
];

function initialLocale(): LocaleId {
  try {
    const saved = localStorage.getItem("lapapp-locale");
    if (saved === "zh" || saved === "en") return saved;
  } catch {
    /* ignore */
  }
  return "zh";
}

export const i18n = createI18n({
  legacy: false,
  locale: initialLocale(),
  fallbackLocale: "en",
  messages: { zh, en },
});

export function applyLocale(locale: string): void {
  i18n.global.locale.value = locale as LocaleId;
  document.documentElement.lang = locale === "zh" ? "zh-CN" : "en";
  try {
    localStorage.setItem("lapapp-locale", locale);
  } catch {
    /* ignore */
  }
}
