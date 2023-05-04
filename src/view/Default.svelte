<script lang="ts">
	import { faCog, faSpinner } from "@fortawesome/free-solid-svg-icons";
	import { faGithub } from "@fortawesome/free-brands-svg-icons";
	import { fs, path } from "@tauri-apps/api";
	import humanize from "humanize-duration";
	import { fade } from "svelte/transition";
	import Icon from "svelte-fa";
	import { _, locale } from "svelte-i18n";
	import { link } from "svelte-navigator";
	import { settings } from "$lib/settings";
	import { updateAvailable, runGame } from "$lib/tauri";

	let playing: boolean = false;
	let hasGame = gameAvailable();
	let secondsPlayed = settings.settings.secondsPlayed;

	async function gameAvailable(): Promise<boolean> {
		return fs.exists(await path.join(settings.getCache("gameDir"), "HoloCure.exe"));
	}

	async function launch() {
		playing = true;
		const timePlayed = await runGame();
		const currentTimePlayed = settings.getCache("secondsPlayed");
		secondsPlayed = currentTimePlayed + timePlayed;
		settings.set("secondsPlayed", secondsPlayed);
		playing = false;
	}
</script>

<div in:fade|local={{ duration: 200 }} class="grid w-1/2 flex-1 grid-cols-3 items-center gap-4 mb-2">
	{#await Promise.all([hasGame, updateAvailable()])}
		<button disabled class="btn">
			<Icon icon={faSpinner} class="icon icon-lg mx-auto animate-spin" />
		</button>
		<button disabled class="btn">
			<Icon icon={faSpinner} class="icon icon-lg mx-auto animate-spin" />
		</button>
	{:then [game, update]}
		<button on:click={launch} disabled={!game || playing} class="btn">
			{playing ? $_("button.playing") : game ? $_("button.play") : $_("button.not_found")}
		</button>
		{#if game}
			<button disabled={!update} class="btn">{update ? $_("button.update_needed") : $_("button.update_not_needed")}</button>
		{:else}
			<a use:link href="/download" tabindex="-1">
				<button class="btn">{$_("button.download")}</button>
			</a>
		{/if}
	{/await}
	<div class="flex items-center gap-4">
		<a href="/settings" use:link tabindex="-1">
			<button class="btn w-fit"><Icon icon={faCog} class="icon icon-lg" /></button>
		</a>
		<a href="https://github.com/TehNut" target="_blank" tabindex="-1">
			<button class="btn w-fit"><Icon icon={faGithub} class="icon icon-lg" /></button>
		</a>
	</div>
</div>
<span class="absolute bottom-0 left-1 font-medium text-sm text-white">
	{$_("time_played", { values: { time: humanize(secondsPlayed * 1000, { language: $locale.split("-")[0], largest: 3 }) } })}
</span>