// examples/common/mod.rs
pub mod common;
use ali_oss::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	common::load_env()?;

	let oss_client = crate::Client::from_env()?;
	let (bytes, headers) = oss_client.get_object("1.html").await?;
	println!("bytes: {:?}, headers: {:?}", bytes, headers);

	Ok(())
}
