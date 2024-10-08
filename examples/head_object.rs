// examples/common/mod.rs
pub mod common;
use ali_oss::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	common::load_env()?;

	let oss_client = crate::Client::from_env()?;
	let headers = oss_client.head_object("lib.rs").await?;
	println!("head_object headers: {:?}", headers);

	Ok(())
}
