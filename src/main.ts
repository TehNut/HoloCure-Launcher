import { register, init as initLocale, getLocaleFromNavigator } from "svelte-i18n";
import "./app.postcss";
import App from "./App.svelte";
import "$lib/settings"; // Init settings

// Initialize i18n
register("en", () => import("$locale/en.json"));
await initLocale({
	fallbackLocale: "en",
	initialLocale: getLocaleFromNavigator(),
});

const app = new App({
	target: document.getElementById("app"),
});

export default app;
