#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

use std::{fs::read, io::Cursor, path::PathBuf, process::Command};

use chrono::{TimeZone, Utc};
use reqwest::get;
use specta::collect_types;
use tauri::{AppHandle, PathResolver};
use tauri_specta::ts;

#[derive(serde::Deserialize)]
struct ItchUploadsResponse {
	uploads: Vec<ItchUpload>,
}

#[derive(serde::Deserialize)]
#[allow(unused)]
struct ItchUpload {
	id: i32,
	created_at: String,
	updated_at: String,
	size: i32,
}

#[derive(serde::Deserialize)]
struct ItchDownloadLinkResponse {
	url: String,
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
async fn download_latest(app: AppHandle) -> Result<bool, HolocureError> {
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
	println!("{}", &download_url);
	let download_url = serde_json::from_str::<ItchDownloadLinkResponse>(&download_url)?;
	let download_url = download_url.url;

	// Download and extract to install dir
	let response = get(download_url).await?;
	let zip_bytes = Cursor::new(response.bytes().await?);
	zip_extract::extract(zip_bytes, &PathBuf::from(settings.game_dir), true)?;

	Ok(true)
}

#[tauri::command]
#[specta::specta]
async fn run_game(app: AppHandle) -> Result<(), HolocureError> {
	let settings = read_settings(app.path_resolver())?;
	let executable = PathBuf::from(settings.game_dir).join("HoloCure.exe");

	let mut game_process = Command::new(executable).spawn()?;
	let _result = game_process.wait()?;

	Ok(())
}

fn main() {
	#[cfg(debug_assertions)]
	ts::export(
		collect_types![update_available, download_latest, run_game],
		"../src/lib/tauri.ts",
	)
	.unwrap();

	tauri::Builder::default()
		.plugin(tauri_plugin_persisted_scope::init())
		.plugin(tauri_plugin_window_state::Builder::default().build())
		.invoke_handler(tauri::generate_handler![update_available, download_latest, run_game])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}

#[derive(Debug, thiserror::Error)]
pub enum HolocureError {
	#[error(transparent)]
	GenericError(#[from] std::io::Error),
	#[error(transparent)]
	RequestError(#[from] reqwest::Error),
	#[error(transparent)]
	JsonError(#[from] serde_json::Error),
	#[error(transparent)]
	ZipError(#[from] zip_extract::ZipError),
	#[error(transparent)]
	ZipExtractError(#[from] zip_extract::ZipExtractError),
}

impl HolocureError {
	pub fn generic<S>(message: S) -> HolocureError
	where
		S: Into<String>,
	{
		Self::GenericError(std::io::Error::new(std::io::ErrorKind::Other, message.into()))
	}
}

impl serde::Serialize for HolocureError {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::ser::Serializer,
	{
		serializer.serialize_str(self.to_string().as_ref())
	}
}

#[derive(serde::Deserialize)]
#[allow(unused)]
#[serde(rename_all = "camelCase")]
struct HolocureSettings {
	game_dir: String,
	launch_command: String,
	local_updated_date: Option<String>,
	itch_api: HolocureSettingsItch,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct HolocureSettingsItch {
	key: String,
	game_id: i32,
}

fn read_settings(path_resolver: PathResolver) -> Result<HolocureSettings, HolocureError> {
	let path = path_resolver.app_data_dir().unwrap().join("settings.json");
	let file = read(path)?;
	let settings = serde_json::from_slice::<HolocureSettings>(file.as_ref())?;

	Ok(settings)
}
