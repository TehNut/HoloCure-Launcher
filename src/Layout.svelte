<script lang="ts">
	import { Route, useLocation } from "svelte-navigator";
	import DefaultView from "./view/Default.svelte";
	import SettingsView from "./view/Settings.svelte";
	import DownloadView from "./view/Download.svelte";

	const location = useLocation();

	$: isHome = $location.pathname === "/";
</script>

<main class="relative z-10 flex min-h-screen flex-col items-center justify-between">
	<div class="{isHome ? 'h-80' : 'h-40'} flex-none transition-[height]">
		<img class="h-full drop-shadow-lg pointer-events-none" src="/logo.png" alt="HoloCure Logo" />
	</div>
	<Route path="/" component={DefaultView} />
	<Route path="/settings" component={SettingsView} />
	<Route path="/download" component={DownloadView} />
</main>
<div class="absolute inset-0">
	<div class="absolute inset-0 bg-[#50c0f9]" />
	<div class="stripes absolute inset-0 top-auto h-1/2 opacity-5" />
	<div class="absolute inset-0 bg-gradient-to-b from-[#50c0f9] via-[#50c0f9] via-70% to-black/10" />
</div>

<style>
	.stripes {
		background: repeating-linear-gradient(135deg, white, white 6px, black 6px, black 18px);
	}
</style>
