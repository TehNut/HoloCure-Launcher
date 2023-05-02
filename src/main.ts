import "./app.postcss";
import App from "./App.svelte";
import "$lib/settings"; // Init settings

const app = new App({
	target: document.getElementById("app"),
});

export default app;
