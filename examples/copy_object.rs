// examples/common/mod.rs
pub mod common;
use ali_oss::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	common::load_env()?;

	let oss_client = crate::Client::from_env()?;
	let headers = oss_client.copy_object("/2.html", "/1.html").await?;
	println!("copy_object headers: {:?}", headers);

	Ok(())
}
