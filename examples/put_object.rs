// examples/common/mod.rs
pub mod common;
use ali_oss::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	common::load_env()?;

	let oss_client = crate::Client::from_env()?;
	let file_path = std::env::current_dir()?.join("src/lib.rs");
	let buffer = std::fs::read(file_path)?;
	let headers = oss_client.put_object("/lib.rs", buffer).await?;
	println!("put_object headers: {:?}", headers);

	Ok(())
}
