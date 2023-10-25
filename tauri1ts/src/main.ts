import { createApp } from "vue";
import { createPinia } from "pinia";
import { createPersistedState } from "pinia-plugin-persistedstate";
import "normalize.css";
import "./styles.css";
import App from "./App.vue";
import router from './router';

const persist = createPersistedState({
    storage: localStorage,
    key: i => `__store__${i}`,
});
const pinia = createPinia();
const app = createApp(App)

pinia.use(persist);
app.use(pinia);
app.use(router);
app.mount("#app");
