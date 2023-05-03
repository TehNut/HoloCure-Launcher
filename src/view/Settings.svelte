<script lang="ts">
	import { faUndo, faFolder, faMagnifyingGlass } from "@fortawesome/free-solid-svg-icons";
	import { dialog } from "@tauri-apps/api";
	import { fade } from "svelte/transition";
	import Icon from "svelte-fa";
	import { _ } from "svelte-i18n";
	import { navigate } from "svelte-navigator";
	import { settings } from "$lib/settings";
	import { openDir } from "$lib/tauri";

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
		<span class="mb-1 ml-4 text-sm font-medium">{$_("settings.install_dir")}</span>
		<input class="input" type="text" bind:value={settings.settings.gameDir} />
	</label>
	<div class="flex items-center gap-4 self-end">
		<button class="btn" on:click={pickGameDir}>
			<Icon icon={faMagnifyingGlass} class="icon icon-lg" />
		</button>
		<button class="btn" on:click={() => openDir(settings.settings.gameDir)}>
			<Icon icon={faFolder} class="icon icon-lg" />
		</button>
		<button class="btn" on:click={() => (settings.settings.gameDir = settings.default.gameDir)}>
			<Icon icon={faUndo} class="icon icon-lg" />
		</button>
	</div>
	<span />
	<button class="btn" on:click={save}>{$_("button.save")}</button>
	<button class="btn" on:click={cancel}>{$_("button.cancel")}</button>
	<span />
</div>
