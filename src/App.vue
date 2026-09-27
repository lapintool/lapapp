<script setup lang="ts">
import { computed, onMounted, ref, type Component } from "vue";
import SideNav from "./components/SideNav.vue";
import TitleBar from "./components/TitleBar.vue";
import { initSettings } from "./settings";
import SettingsView from "./views/SettingsView.vue";
import WidgetsView from "./views/WidgetsView.vue";

const pages: Record<string, Component> = {
  widgets: WidgetsView,
  settings: SettingsView,
};

const active = ref<string>("widgets");
const activeView = computed(() => pages[active.value] ?? WidgetsView);

onMounted(() => {
  void initSettings();
});
</script>

<template>
  <div class="app-shell">
    <TitleBar />
    <div class="app-body">
      <SideNav v-model="active" />
      <main class="app-main ls-scroll">
        <component :is="activeView" />
      </main>
    </div>
  </div>
</template>
