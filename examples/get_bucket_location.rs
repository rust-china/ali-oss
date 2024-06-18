// examples/common/mod.rs
pub mod common;
use ali_oss::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	common::load_env()?;

	let oss_client = crate::Client::from_env()?;
	let bucket_location = oss_client.get_bucket_location().await?;
	println!("bucket_location: {:?}", bucket_location);

	Ok(())
}
