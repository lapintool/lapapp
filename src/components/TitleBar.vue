<script setup lang="ts">
import { onMounted, onUnmounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { useI18n } from "vue-i18n";
import { LOCALES } from "../i18n";
import { THEMES, locale, setLocale, setTheme, theme, type ThemeId } from "../settings";
import appIcon from "../assets/app-icon.png";

const { t } = useI18n();

interface MenuSelectDetail {
  value?: string | number | null;
  label?: string;
}

const custom = ref(false);
const maximized = ref(false);

const MIN_ICON = `<svg viewBox="0 0 12 12" width="12" height="12" aria-hidden="true" fill="none" stroke="currentColor" stroke-width="1" stroke-linecap="square"><path d="M2.5 6h7"/></svg>`;
const MAX_ICON = `<svg viewBox="0 0 12 12" width="12" height="12" aria-hidden="true" fill="none" stroke="currentColor" stroke-width="1"><rect x="2.5" y="2.5" width="7" height="7"/></svg>`;
const RESTORE_ICON = `<svg viewBox="0 0 12 12" width="12" height="12" aria-hidden="true" fill="none" stroke="currentColor" stroke-width="1"><path d="M4 3.5h4.5V8"/><rect x="2.5" y="4.5" width="5.5" height="5.5"/></svg>`;
const CLOSE_ICON = `<svg viewBox="0 0 12 12" width="12" height="12" aria-hidden="true" fill="none" stroke="currentColor" stroke-width="1" stroke-linecap="square"><path d="M3 3l6 6M9 3L3 9"/></svg>`;
const GLOBE_ICON = `<svg viewBox="0 0 16 16" width="16" height="16" aria-hidden="true" fill="none" stroke="currentColor" stroke-width="1.3"><circle cx="8" cy="8" r="6.2"/><path d="M1.8 8h12.4M8 1.8c-4.7 4.1-4.7 8.3 0 12.4M8 1.8c4.7 4.1 4.7 8.3 0 12.4"/></svg>`;
const MOON_ICON = `<svg viewBox="0 0 16 16" width="16" height="16" aria-hidden="true" fill="none" stroke="currentColor" stroke-width="1.3" stroke-linejoin="round"><path d="M13.2 9.7A5.5 5.5 0 0 1 6.3 2.8a5.5 5.5 0 1 0 6.9 6.9Z"/></svg>`;

let unlistenResized: (() => void) | null = null;

async function syncMaximized(): Promise<void> {
  try {
    maximized.value = await getCurrentWindow().isMaximized();
  } catch {
    /* ignore */
  }
}

function minimize(): void {
  void getCurrentWindow().minimize();
}

function toggleMaximize(): void {
  void getCurrentWindow().toggleMaximize();
}

function closeWindow(): void {
  void getCurrentWindow().close();
}

function onSelectLocale(detail: MenuSelectDetail): void {
  void setLocale(String(detail.value ?? ""));
}

function onSelectTheme(detail: MenuSelectDetail): void {
  const id = String(detail.value ?? "");
  if ((THEMES as readonly string[]).includes(id)) void setTheme(id as ThemeId);
}

onMounted(async () => {
  try {
    custom.value = await invoke<boolean>("uses_custom_titlebar");
  } catch {
    custom.value = false;
  }
  if (!custom.value) return;
  void syncMaximized();
  try {
    unlistenResized = await getCurrentWindow().onResized(() => {
      void syncMaximized();
    });
  } catch {
    /* ignore */
  }
});

onUnmounted(() => {
  unlistenResized?.();
  unlistenResized = null;
});
</script>

<template>
  <header class="titlebar" data-tauri-drag-region>
    <div class="titlebar-brand" data-tauri-drag-region>
      <img class="brand-icon" :src="appIcon" alt="" draggable="false" />
      <span>{{ t("app.name") }}</span>
    </div>

    <div class="titlebar-chrome">
      <!-- 语言菜单：无背景无边框的圆形图标按钮 -->
      <ls-btn-dropdown
        class="chrome-dd"
        variant="ghost"
        :label="t('menu.language')"
        @select="onSelectLocale"
      >
        <template #label>
          <span class="chrome-icon" v-html="GLOBE_ICON"></span>
        </template>
        <button
          v-for="item in LOCALES"
          :key="item.id"
          type="button"
          class="item"
          :class="{ 'is-active': locale === item.id }"
          :data-value="item.id"
        >
          <span class="label">{{ item.label }}</span>
        </button>
      </ls-btn-dropdown>

      <!-- 主题菜单：无背景无边框的圆形图标按钮 -->
      <ls-btn-dropdown
        class="chrome-dd"
        variant="ghost"
        :label="t('menu.theme')"
        @select="onSelectTheme"
      >
        <template #label>
          <span class="chrome-icon" v-html="MOON_ICON"></span>
        </template>
        <button
          v-for="id in THEMES"
          :key="id"
          type="button"
          class="item"
          :class="{ 'is-active': theme === id }"
          :data-value="id"
        >
          <span class="label">{{ t("theme." + id) }}</span>
        </button>
      </ls-btn-dropdown>

      <!-- 三个窗口按钮（仅自绘标题栏平台显示） -->
      <div v-if="custom" class="window-controls">
        <button type="button" :title="t('window.minimize')" @click="minimize()">
          <span class="chrome-icon" v-html="MIN_ICON"></span>
        </button>
        <button
          type="button"
          :title="maximized ? t('window.restore') : t('window.maximize')"
          @click="toggleMaximize()"
        >
          <span class="chrome-icon" v-html="maximized ? RESTORE_ICON : MAX_ICON"></span>
        </button>
        <button type="button" class="close" :title="t('window.close')" @click="closeWindow()">
          <span class="chrome-icon" v-html="CLOSE_ICON"></span>
        </button>
      </div>
    </div>
  </header>
</template>
