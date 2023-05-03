import { path } from "@tauri-apps/api";
import { SettingsManager } from "tauri-settings";

export type LauncherSettings = {
	gameDir: string;
	launchCommand: string;
	secondsPlayed: number,
	localUpdatedDate: string | null;
	itchApi: {
		key: string;
		gameId: number;
	};
};

export const settings = new SettingsManager<LauncherSettings>(
	{
		gameDir: await path.join(await path.appDataDir(), "Game"),
		launchCommand: "",
		secondsPlayed: 0,
		localUpdatedDate: null,
		itchApi: {
			// Key from the official launcher config file. Goes to a throwaway HoloCure account.
			key: "l7McEiuy5JD9OxqKfiO6gnlT2Obv9rBUpIKGBloH",
			gameId: 1576684,
		},
	},
	{
		dir: await path.appDataDir(),
		prettify: true,
		numSpaces: 2,
	}
);

await settings.initialize();
