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
