<script setup lang="ts">
import { useI18n } from "vue-i18n";

const model = defineModel<string>({ required: true });
const { t } = useI18n();
</script>

<template>
  <aside class="side-nav">
    <ls-menu v-model="model" plain :card="false">
      <ls-menu-item value="widgets">
        <template #icon>
          <svg
            viewBox="0 0 16 16"
            width="16"
            height="16"
            fill="none"
            stroke="currentColor"
            stroke-width="1.4"
            stroke-linejoin="round"
          >
            <rect x="2.5" y="2.5" width="4.6" height="4.6" rx="0.8" />
            <rect x="8.9" y="2.5" width="4.6" height="4.6" rx="0.8" />
            <rect x="2.5" y="8.9" width="4.6" height="4.6" rx="0.8" />
            <rect x="8.9" y="8.9" width="4.6" height="4.6" rx="0.8" />
          </svg>
        </template>
        {{ t("nav.widgets") }}
      </ls-menu-item>

      <ls-menu-item value="settings">
        <template #icon>
          <svg
            viewBox="0 0 16 16"
            width="16"
            height="16"
            fill="none"
            stroke="currentColor"
            stroke-width="1.4"
            stroke-linecap="round"
          >
            <path d="M2.5 4.5h11M2.5 8h11M2.5 11.5h11" />
            <circle cx="6" cy="4.5" r="1.4" />
            <circle cx="10.2" cy="8" r="1.4" />
            <circle cx="5.2" cy="11.5" r="1.4" />
          </svg>
        </template>
        {{ t("nav.settings") }}
      </ls-menu-item>
    </ls-menu>
  </aside>
</template>
