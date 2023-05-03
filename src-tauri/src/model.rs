#[derive(serde::Deserialize)]
pub struct ItchUploadsResponse {
	pub uploads: Vec<ItchUpload>,
}

#[derive(serde::Deserialize)]
#[allow(unused)]
pub struct ItchUpload {
	pub id: i32,
	pub created_at: String,
	pub updated_at: String,
	pub size: i32,
}

#[derive(serde::Deserialize)]
pub struct ItchDownloadLinkResponse {
	pub url: String,
}

#[derive(Debug, thiserror::Error)]
pub enum HolocureError {
	#[error(transparent)]
	GenericError(#[from] std::io::Error),
	#[error(transparent)]
	TauriError(#[from] tauri::Error),
	#[error(transparent)]
	RequestError(#[from] reqwest::Error),
	#[error(transparent)]
	JsonError(#[from] serde_json::Error),
	#[error(transparent)]
	ZipError(#[from] zip_extract::ZipError),
	#[error(transparent)]
	ZipExtractError(#[from] zip_extract::ZipExtractError),
}

#[allow(unused)]
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
pub struct HolocureSettings {
	pub game_dir: String,
	pub launch_command: String,
	pub local_updated_date: Option<String>,
	pub itch_api: HolocureSettingsItch,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HolocureSettingsItch {
	pub key: String,
	pub game_id: i32,
}
