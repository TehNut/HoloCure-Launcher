<script lang="ts">
	import { faUndo, faFolder, faMagnifyingGlass } from "@fortawesome/free-solid-svg-icons";
	import { dialog, shell } from "@tauri-apps/api";
	import { fade } from "svelte/transition";
	import Icon from "svelte-fa";
	import { navigate } from "svelte-navigator";
	import { settings } from "$lib/settings";

	async function pickGameDir() {
		const selection = await dialog.open({
			defaultPath: settings.settings.gameDir,
			directory: true,
			title: "Game installation path",
		});

		if (selection) settings.settings.gameDir = selection as string;
	}

	async function cancel() {
		await settings.initialize();
		navigate("/");
	}

	async function save() {
		await settings.syncCache();
		navigate("/");
	}
</script>

<div out:fade|local={{ duration: 200 }} class="grid flex-1 grid-cols-4 items-center gap-x-4">
	<label class="col-span-3 flex flex-col justify-between self-end">
		<span class="mb-1 ml-4 text-sm font-medium group-hover:mb-px">Install Directory</span>
		<input class="input" type="text" bind:value={settings.settings.gameDir} />
	</label>
	<div class="flex items-center gap-4 self-end">
		<button class="btn" on:click={pickGameDir}>
			<Icon icon={faMagnifyingGlass} size="lg" />
		</button>
		<button class="btn" on:click={() => shell.open("file://" + settings.settings.gameDir)}>
			<Icon icon={faFolder} size="lg" />
		</button>
		<button class="btn" on:click={() => (settings.settings.gameDir = settings.default.gameDir)}>
			<Icon icon={faUndo} size="lg" />
		</button>
	</div>
	<span />
	<button class="btn" on:click={save}>Save</button>
	<button class="btn" on:click={cancel}>Cancel</button>
	<span />
</div>
