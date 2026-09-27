<script setup lang="ts">
import { onMounted, ref } from "vue";
import { getVersion } from "@tauri-apps/api/app";
import { useI18n } from "vue-i18n";
import { LOCALES } from "../i18n";
import { THEMES, locale, setLocale, setTheme, theme } from "../settings";

const { t } = useI18n();

const version = ref("");

onMounted(async () => {
  try {
    version.value = (await getVersion()) || "";
  } catch {
    /* ignore */
  }
});
</script>

<template>
  <section class="page">
    <h1 class="page-title">{{ t("settings.title") }}</h1>

    <ls-card class="settings-group">
      <h3 class="card-title">{{ t("settings.appearance") }}</h3>
      <div class="setting-row stack">
        <div class="setting-text">
          <span class="setting-label">{{ t("settings.theme") }}</span>
          <span class="setting-desc">{{ t("settings.themeDesc") }}</span>
        </div>
        <div class="theme-swatches" role="radiogroup" :aria-label="t('settings.theme')">
          <button
            v-for="id in THEMES"
            :key="id"
            type="button"
            role="radio"
            class="theme-swatch"
            :class="{ 'is-active': theme === id }"
            :aria-checked="theme === id"
            @click="setTheme(id)"
          >
            <span class="theme-swatch__preview" :data-theme="id" aria-hidden="true">
              <span class="theme-swatch__side"></span>
              <span class="theme-swatch__body">
                <span class="theme-swatch__card"></span>
                <span class="theme-swatch__accent"></span>
              </span>
            </span>
            <span>{{ t("theme." + id) }}</span>
          </button>
        </div>
      </div>
    </ls-card>

    <ls-card class="settings-group">
      <h3 class="card-title">{{ t("settings.general") }}</h3>
      <div class="setting-row">
        <div class="setting-text">
          <span class="setting-label">{{ t("settings.language") }}</span>
          <span class="setting-desc">{{ t("settings.languageDesc") }}</span>
        </div>
        <div class="setting-control">
          <ls-dropdown
            :model-value="locale"
            :options="LOCALES.map((item) => ({ value: item.id, label: item.label }))"
            @update:model-value="setLocale(String($event))"
          />
        </div>
      </div>
    </ls-card>

    <ls-card class="settings-group">
      <h3 class="card-title">{{ t("settings.about") }}</h3>
      <div class="setting-row">
        <div class="setting-text">
          <span class="setting-label">{{ t("app.name") }}</span>
          <span class="setting-desc">{{ t("settings.aboutDesc") }}</span>
        </div>
      </div>
      <div class="setting-row">
        <span class="setting-label">{{ t("settings.version") }}</span>
        <code class="setting-code">{{ version || "?" }}</code>
      </div>
      <div class="setting-row">
        <span class="setting-label">{{ t("settings.stack") }}</span>
        <code class="setting-code">Tauri 2 · Vue 3 · lapstyle</code>
      </div>
    </ls-card>
  </section>
</template>
