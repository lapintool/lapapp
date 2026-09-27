import { createApp } from "vue";
import { LapstyleVue } from "lapstyle/vue";
import "lapstyle/index.css";
import "./styles.css";
import App from "./App.vue";
import { i18n } from "./i18n";
import { bindWindowStateSave } from "./windowState";
import { bindZoomShortcuts } from "./zoom";

createApp(App).use(i18n).use(LapstyleVue).mount("#app");

bindWindowStateSave();
bindZoomShortcuts();
