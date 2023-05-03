<script lang="ts">
	import { faCog, faSpinner } from "@fortawesome/free-solid-svg-icons";
	import { faGithub } from "@fortawesome/free-brands-svg-icons";
	import { fs, path } from "@tauri-apps/api";
	import { fade } from "svelte/transition";
	import Icon from "svelte-fa";
	import { _ } from "svelte-i18n";
	import { link } from "svelte-navigator";
	import { settings } from "$lib/settings";
	import { updateAvailable, runGame } from "$lib/tauri";

	let playing: boolean = false;
	let hasGame = gameAvailable();

	async function gameAvailable(): Promise<boolean> {
		return fs.exists(await path.join(settings.getCache("gameDir"), "HoloCure.exe"));
	}

	async function launch() {
		playing = true;
		await runGame();
		playing = false;
	}
</script>

<div in:fade|local={{ duration: 200 }} class="grid w-1/2 flex-1 grid-cols-3 items-center gap-4">
	{#await Promise.all([hasGame, updateAvailable()])}
		<button disabled class="btn">
			<Icon icon={faSpinner} size="lg" class="mx-auto animate-spin" />
		</button>
		<button disabled class="btn">
			<Icon icon={faSpinner} size="lg" class="mx-auto animate-spin" />
		</button>
	{:then [game, update]}
		<button on:click={launch} disabled={!game || playing} class="btn">
			{playing ? $_("button.playing") : game ? $_("button.play") : $_("button.not_found")}
		</button>
		{#if game}
			<button disabled={!update} class="btn">{update ? $_("button.update_needed") : $_("button.update_not_needed")}</button>
		{:else}
			<a use:link href="/download">
				<button class="btn">{$_("button.download")}</button>
			</a>
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
