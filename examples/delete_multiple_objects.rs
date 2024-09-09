// examples/common/mod.rs
pub mod common;
use ali_oss::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	common::load_env()?;

	let oss_client = crate::Client::from_env()?;
	oss_client.delete_multiple_objects(vec!["lib.rs", "lib2.rs"]).await?;
	println!("delete_object success");

	Ok(())
}
