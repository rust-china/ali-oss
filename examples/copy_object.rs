// examples/common/mod.rs
pub mod common;
use ali_oss::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	common::load_env()?;

	let oss_client = crate::Client::from_env()?;
	let headers = oss_client.copy_object("/1.html", "/2.html").await?;
	println!("copy_object headers: {:?}", headers);

	Ok(())
}
