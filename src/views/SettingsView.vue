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

    <ls-card class="settings-card">
      <h3 class="card-title">{{ t("settings.appearance") }}</h3>
      <ls-field :label="t('settings.theme')">
        <div class="row wrap">
          <ls-btn
            v-for="id in THEMES"
            :key="id"
            size="sm"
            :variant="theme === id ? 'fill' : ''"
            @click="setTheme(id)"
          >
            {{ t("theme." + id) }}
          </ls-btn>
        </div>
      </ls-field>
    </ls-card>

    <ls-card class="settings-card">
      <h3 class="card-title">{{ t("settings.general") }}</h3>
      <ls-field :label="t('settings.language')">
        <ls-dropdown
          :model-value="locale"
          :options="LOCALES.map((item) => ({ value: item.id, label: item.label }))"
          @update:model-value="setLocale(String($event))"
        />
      </ls-field>
    </ls-card>

    <ls-card class="settings-card">
      <h3 class="card-title">{{ t("settings.about") }}</h3>
      <p class="muted">{{ t("settings.aboutDesc") }}</p>
      <p class="kv"><span>{{ t("settings.version") }}</span><code>{{ version || "?" }}</code></p>
      <p class="kv"><span>{{ t("settings.stack") }}</span><code>Tauri 2 · Vue 3 · lapstyle</code></p>
    </ls-card>
  </section>
</template>
