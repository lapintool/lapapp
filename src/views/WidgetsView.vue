<script setup lang="ts">
import { ref } from "vue";
import { useI18n } from "vue-i18n";

const { t } = useI18n();

const name = ref("");
const notifications = ref(true);
const plan = ref("standard");
const volume = ref(60);
const progress = ref(72);
const option = ref("a");
const tab = ref("overview");

const optionItems = [
  { value: "a", label: "Option A" },
  { value: "b", label: "Option B" },
  { value: "c", label: "Option C" },
];
</script>

<template>
  <section class="page wide">
    <h1 class="page-title">{{ t("widgets.title") }}</h1>
    <p class="page-sub">{{ t("widgets.subtitle") }}</p>

    <div class="widget-grid">
      <ls-card>
        <h3 class="card-title">{{ t("widgets.buttons") }}</h3>
        <div class="row wrap">
          <ls-btn>{{ t("widgets.default") }}</ls-btn>
          <ls-btn variant="fill" color="blue">{{ t("widgets.fill") }}</ls-btn>
          <ls-btn variant="ghost">{{ t("widgets.ghost") }}</ls-btn>
          <ls-btn variant="push" color="green">Push</ls-btn>
          <ls-btn disabled>{{ t("widgets.disabled") }}</ls-btn>
          <ls-btn>
            <ls-icon>
              <svg
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
                aria-hidden="true"
              >
                <path d="M12 3v12M7 10l5 5 5-5M5 21h14" />
              </svg>
            </ls-icon>
            {{ t("widgets.download") }}
          </ls-btn>
          <ls-btn round icon variant="flat" color="cyan" :title="t('nav.settings')" :aria-label="t('nav.settings')">
            <ls-icon>
              <svg
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
                aria-hidden="true"
              >
                <path
                  d="M9.671 4.136a2.34 2.34 0 0 1 4.659 0 2.34 2.34 0 0 0 3.319 1.915 2.34 2.34 0 0 1 2.33 4.033 2.34 2.34 0 0 0 0 3.831 2.34 2.34 0 0 1-2.33 4.033 2.34 2.34 0 0 0-3.319 1.915 2.34 2.34 0 0 1-4.659 0 2.34 2.34 0 0 0-3.32-1.915 2.34 2.34 0 0 1-2.33-4.033 2.34 2.34 0 0 0 0-3.831A2.34 2.34 0 0 1 6.35 6.051a2.34 2.34 0 0 0 3.319-1.915"
                />
                <circle cx="12" cy="12" r="3" />
              </svg>
            </ls-icon>
          </ls-btn>
        </div>
      </ls-card>

      <ls-card>
        <h3 class="card-title">{{ t("widgets.inputs") }}</h3>
        <div class="col">
          <ls-field :label="t('widgets.nickname')">
            <ls-input v-model="name" clearable :placeholder="t('widgets.nicknamePh')" />
          </ls-field>
          <ls-checkbox v-model="notifications" :label="t('widgets.notifications')" />
          <div class="row">
            <ls-radio v-model="plan" value="standard" :label="t('widgets.standard')" />
            <ls-radio v-model="plan" value="pro" :label="t('widgets.pro')" />
          </div>
        </div>
      </ls-card>

      <ls-card>
        <h3 class="card-title">{{ t("widgets.controls") }}</h3>
        <div class="col">
          <div class="ctl-row">
            <span class="ctl-label">{{ t("widgets.volume") }}</span>
            <ls-slider v-model="volume" :min="0" :max="100" label />
          </div>
          <div class="ctl-row">
            <span class="ctl-label">Progress</span>
            <ls-progress v-model="progress" />
          </div>
          <div class="ctl-row">
            <span class="ctl-label">{{ t("widgets.option") }}</span>
            <ls-dropdown v-model="option" :options="optionItems" />
          </div>
        </div>
      </ls-card>

      <ls-card>
        <h3 class="card-title">{{ t("widgets.tabs") }}</h3>
        <ls-tabs v-model="tab" bar-only>
          <ls-tab value="overview" :label="t('widgets.overview')" />
          <ls-tab value="activity" :label="t('widgets.activity')" />
          <ls-tab value="logs" :label="t('widgets.logs')" />
        </ls-tabs>
        <p class="muted">{{ t(`widgets.${tab}Body`) }}</p>
      </ls-card>
    </div>
  </section>
</template>
