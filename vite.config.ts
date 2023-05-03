import { fileURLToPath, URL } from "node:url";
import { defineConfig } from "vite";
import { svelte, vitePreprocess } from "@sveltejs/vite-plugin-svelte";

export default defineConfig({
	resolve: {
		alias: {
			$lib: fileURLToPath(new URL("./src/lib", import.meta.url)),
			$locale: fileURLToPath(new URL("./locale", import.meta.url)),
		},
	},
	build: {
		target: "esnext",
		outDir: "build",
	},
	optimizeDeps: {
		exclude: ["svelte-navigator"],
	},
	server: {
		strictPort: true,
	},
	plugins: [
		svelte({
			preprocess: vitePreprocess(),
		}),
	],
});
