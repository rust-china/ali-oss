// examples/common/mod.rs
pub mod common;

use ali_oss::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	common::load_env()?;

	let oss_client = crate::Client::from_env()?;
	let buckets = oss_client.list_buckets().await?;
	println!("buckets: {:?}", buckets);

	Ok(())
}
