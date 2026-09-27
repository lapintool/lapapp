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

/// Material Symbols outlined（weight 400），viewBox "0 -960 960 960"，fill 渲染
const TRANSLATE_PATH = `m475-80 185-480h79L924-80h-65l-45-117H584L539-80h-64ZM162-201l-42-42 201-201q-51-53-85.5-107.5T183-660h65q16 43 43.5 85t72.5 88q46-48 85-117.5T505-740H40v-60h290v-80h60v80h290v60H567q-17 78-61.5 159.5T406-443l102 104-24 63-121-125-201 200Zm443-51h188l-94-248-94 248Z`;
const PALETTE_PATH = `M480-80q-82 0-155-31.5t-127.5-86Q143-252 111.5-325T80-480q0-85 32-158t87.5-127q55.5-54 130-84.5T489-880q79 0 150 26.5T763.5-780q53.5 47 85 111.5T880-527q0 108-63 170.5T650-294h-75q-18 0-31 14t-13 31q0 27 14.5 46t14.5 44q0 38-21 58.5T480-80Zm0-400Zm-198 11q15-15 15-35t-15-35q-15-15-35-15t-35 15q-15 15-15 35t15 35q15 15 35 15t35-15Zm126-170q15-15 15-35t-15-35q-15-15-35-15t-35 15q-15 15-15 35t15 35q15 15 35 15t35-15Zm214 0q15-15 15-35t-15-35q-15-15-35-15t-35 15q-15 15-15 35t15 35q15 15 35 15t35-15Zm131 170q15-15 15-35t-15-35q-15-15-35-15t-35 15q-15 15-15 35t15 35q15 15 35 15t35-15ZM480-140q11 0 15.5-4.5T500-159q0-14-14.5-26T471-238q0-46 30-81t76-35h73q76 0 123-44.5T820-527q0-132-100-212.5T489-820q-146 0-247.5 98.5T140-480q0 141 99.5 240.5T480-140Z`;

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
      <!-- 语言菜单：lapstyle 圆形图标按钮（flat 变体 + round/icon 外观） -->
      <ls-btn-dropdown
        class="chrome-dd"
        variant="flat"
        :label="t('menu.language')"
        @select="onSelectLocale"
      >
        <template #label>
          <ls-icon>
            <svg viewBox="0 -960 960 960" fill="currentColor" aria-hidden="true">
              <path :d="TRANSLATE_PATH" />
            </svg>
          </ls-icon>
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

      <!-- 主题菜单：lapstyle 圆形图标按钮（flat 变体 + round/icon 外观） -->
      <ls-btn-dropdown
        class="chrome-dd"
        variant="flat"
        :label="t('menu.theme')"
        @select="onSelectTheme"
      >
        <template #label>
          <ls-icon>
            <svg viewBox="0 -960 960 960" fill="currentColor" aria-hidden="true">
              <path :d="PALETTE_PATH" />
            </svg>
          </ls-icon>
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
