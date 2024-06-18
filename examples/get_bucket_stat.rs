// examples/common/mod.rs
pub mod common;
use ali_oss::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	common::load_env()?;

	let oss_client = crate::Client::from_env()?;
	let bucket_stat = oss_client.get_bucket_stat().await?;
	println!("bucket_stat: {:?}", bucket_stat);

	Ok(())
}
