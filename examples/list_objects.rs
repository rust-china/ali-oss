// examples/common/mod.rs
pub mod common;

use ali_oss::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	common::load_env()?;

	let oss_client = crate::Client::from_env()?;
	let (folders, files) = oss_client.list_objects(None, Some("/")).await?;
	println!("folders: {:?}, files: {:?}", folders, files);

	let folders = oss_client.list_folders(None).await?;
	println!("folders: {:?}", folders);

	let files = oss_client.list_files(None).await?;
	println!("files: {:?}", files);

	Ok(())
}
