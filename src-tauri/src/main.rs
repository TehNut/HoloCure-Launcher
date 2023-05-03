#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

mod model;

use std::{fs::read, io::Cursor, path::PathBuf, process::Command};

use chrono::{TimeZone, Utc};
use reqwest::get;
#[cfg(debug_assertions)]
use specta::collect_types;
use tauri::{AppHandle, PathResolver, Window};
#[cfg(debug_assertions)]
use tauri_specta::ts;

use model::*;

fn main() {
	#[cfg(debug_assertions)]
	ts::export(
		collect_types![update_available, get_download_url, extract_update, run_game, open_dir],
		"../src/lib/tauri.ts",
	)
	.unwrap();

	tauri::Builder::default()
		.plugin(tauri_plugin_upload::init())
		.plugin(tauri_plugin_persisted_scope::init())
		.plugin(tauri_plugin_window_state::Builder::default().build())
		.invoke_handler(tauri::generate_handler![
			update_available,
			get_download_url,
			extract_update,
			run_game,
			open_dir
		])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}

#[tauri::command]
#[specta::specta]
async fn update_available(app: AppHandle) -> Result<bool, HolocureError> {
	let settings = read_settings(app.path_resolver())?;
	let uploads = get(format!(
		"https://itch.io/api/1/{}/game/{}/uploads",
		settings.itch_api.key, settings.itch_api.game_id
	))
	.await?
	.text()
	.await?;
	let uploads = serde_json::from_str::<ItchUploadsResponse>(&uploads)?;
	let upload = uploads.uploads.first().unwrap();

	let last_success = match settings.local_updated_date {
		Some(date) => Utc.datetime_from_str(&date, "%Y-%m-%d %H:%M:%S").unwrap(),
		None => return Ok(true),
	};
	let updated_date = Utc.datetime_from_str(&upload.updated_at, "%Y-%m-%d %H:%M:%S").unwrap();
	Ok(updated_date.signed_duration_since(last_success).num_seconds() > 0)
}

#[tauri::command]
#[specta::specta]
async fn get_download_url(app: AppHandle) -> Result<String, HolocureError> {
	let settings = read_settings(app.path_resolver())?;

	// Get Itch data for latest upload
	let uploads = get(format!(
		"https://itch.io/api/1/{}/game/{}/uploads",
		settings.itch_api.key, settings.itch_api.game_id
	))
	.await?
	.text()
	.await?;
	let uploads = serde_json::from_str::<ItchUploadsResponse>(&uploads)?;
	let upload = uploads.uploads.first().unwrap();

	// Get download URL
	let download_url = get(format!(
		"https://itch.io/api/1/{}/upload/{}/download",
		settings.itch_api.key, upload.id
	))
	.await?
	.text()
	.await?;
	let download_url = serde_json::from_str::<ItchDownloadLinkResponse>(&download_url)?;
	let download_url = download_url.url;

	Ok(download_url)
}

#[tauri::command]
#[specta::specta]
async fn extract_update(app: AppHandle) -> Result<(), HolocureError> {
	let settings = read_settings(app.path_resolver())?;
	let file = read(PathBuf::from(app.path_resolver().app_data_dir().unwrap()).join("HoloCure.zip"))?;
	let zip_bytes = Cursor::new(file);
	zip_extract::extract(zip_bytes, &PathBuf::from(settings.game_dir), true)?;

	Ok(())
}

#[tauri::command]
#[specta::specta]
async fn run_game(app: AppHandle, window: Window) -> Result<i32, HolocureError> {
	let settings = read_settings(app.path_resolver())?;
	let executable = PathBuf::from(settings.game_dir).join("HoloCure.exe");

	window.hide()?;
	let game_started_at = Utc::now();
	let mut game_process = Command::new(executable).spawn()?;
	let _result = game_process.wait()?;
	let game_ended_at = Utc::now();
	window.show()?;

	let playtime = game_ended_at.signed_duration_since(game_started_at);

	Ok(i32::try_from(playtime.num_seconds()).ok().unwrap())
}

#[tauri::command]
#[specta::specta]
async fn open_dir(path: &str) -> Result<(), HolocureError> {
	open::that(path)?;
	Ok(())
}

fn read_settings(path_resolver: PathResolver) -> Result<HolocureSettings, HolocureError> {
	let path = path_resolver.app_data_dir().unwrap().join("settings.json");
	let file = read(path)?;
	let settings = serde_json::from_slice::<HolocureSettings>(file.as_ref())?;

	Ok(settings)
}