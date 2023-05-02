<script lang="ts">
	import { faCog, faSpinner } from "@fortawesome/free-solid-svg-icons";
	import { faGithub } from "@fortawesome/free-brands-svg-icons";
	import { fs, path } from "@tauri-apps/api";
	import { fade } from "svelte/transition";
	import Icon from "svelte-fa";
	import { link } from "svelte-navigator";
	import { settings } from "$lib/settings";
	import { downloadLatest, updateAvailable, runGame } from "$lib/tauri";

	let downloading: boolean = false;
	let playing: boolean = false;
	let hasGame = gameAvailable();

	async function gameAvailable(): Promise<boolean> {
		return fs.exists(await path.join(settings.getCache("gameDir"), "HoloCure.exe"));
	}

	async function beginDownload() {
		downloading = true;
		const result = await downloadLatest();
		if (result) {
			downloading = false;
			const [date, time] = new Date().toISOString().split("T");
			settings.settings.localUpdatedDate = `${date} ${time.split(".")[0]}`;
			await settings.syncCache();
			hasGame = Promise.resolve(true);
		}
	}

	async function launch() {
		playing = true;
		await runGame();
		playing = false;
	}
</script>

<div in:fade|local={{ duration: 200 }} class="grid w-1/2 flex-1 grid-cols-3 items-center gap-4">
	{#await Promise.all([hasGame, updateAvailable()])}
		<button disabled class="btn"
			><Icon icon={faSpinner} size="lg" class="mx-auto animate-spin" /></button
		>
		<button disabled class="btn"
			><Icon icon={faSpinner} size="lg" class="mx-auto animate-spin" /></button
		>
	{:then [game, update]}
		<button on:click={launch} disabled={!game || playing} class="btn"
			>{playing ? "Playing..." : game ? "Play" : "Not found"}</button
		>
		{#if game}
			<button disabled={!update} class="btn">{update ? "Update" : "Up to date"}</button>
		{:else}
			<button
				disabled={downloading}
				class="btn {downloading ? '' : 'animate-scale'} hover:animate-none"
				on:click={beginDownload}
			>
				{#if downloading}
					<Icon icon={faSpinner} size="lg" class="mx-auto animate-spin" />
				{:else}
					Download
				{/if}
			</button>
		{/if}
	{/await}
	<div class="flex items-center gap-4">
		<a href="/settings" use:link>
			<button class="btn w-fit"><Icon size="lg" icon={faCog} /></button>
		</a>
		<a href="https://github.com/TehNut" target="_blank">
			<button class="btn w-fit"><Icon size="lg" icon={faGithub} /></button>
		</a>
	</div>
</div>
