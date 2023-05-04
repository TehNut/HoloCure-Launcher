<script lang="ts">
	import { path, fs } from "@tauri-apps/api";
	import { fade } from "svelte/transition";
	import { _ } from "svelte-i18n";
	import { navigate } from "svelte-navigator";
	import { download } from "tauri-plugin-upload-api";
	import { getDownloadUrl, extractUpdate } from "$lib/tauri";

	let state: "initialization" | "download" | "extract" | "cleanup" | "complete" = "initialization";
	let downloadProgress = 0;
	let archiveSize = 0;

	$: progress = archiveSize ? Math.floor((downloadProgress / archiveSize) * 100) : 0;

	getDownloadUrl().then(async (url) => {
		const archivePath = await path.join(await path.appDataDir(), "HoloCure.zip");
		const archiveExists = await fs.exists(archivePath);
		if (!archiveExists) {
			state = "download";
			await download(url, archivePath, (progress, total) => {
				downloadProgress += progress;
				archiveSize = total;
			});
		}
		state = "extract";
		await extractUpdate();
		state = "cleanup";
		await fs.removeFile("HoloCure.zip", { dir: path.BaseDirectory.AppData });
		state = "complete";
		navigate("/");
	});
</script>

<div out:fade|local={{ duration: 200 }} class="flex w-full flex-1 flex-col justify-center gap-y-2">
	<div class="relative mx-14">
		<div class="overflow-none flex h-11 w-full rounded-br-md rounded-tl-md">
			<div
				style="--progress:{progress}%"
				class="h-full border-r-0 bg-white {progress > 0
					? 'border border-b-4 border-black'
					: ''} w-[--progress] rounded-tl-md transition-[width]"
			/>
			<div
				style="--progress:{100 - progress}%"
				class="h-full border-l-0 bg-black/60 {progress < 100
					? 'border border-b-4 border-white'
					: ''} w-[--progress] rounded-br-md transition-[width]"
			/>
		</div>
		<img
			src="/icon.png"
			alt="HoloCure Icon"
			style="--progress:{Math.min(progress - 3.5, 1000)}%"
			class="absolute -top-2.5 left-[--progress] w-16 transition-[left] pointer-events-none"
			class:animate-wiggle={progress < 100}
		/>
	</div>
	<div class="text-center font-medium text-white">
		<span class="capitalize">{$_("download." + state)}</span>
		{#if state === "download"}
			<span class="ml-1">{progress}%</span>
		{/if}
	</div>
</div>
