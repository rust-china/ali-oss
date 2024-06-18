// examples/common/mod.rs
pub mod common;
use ali_oss::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	common::load_env()?;

	let oss_client = crate::Client::from_env()?;
	let headers = oss_client.get_object_meta("1.html").await?;
	println!("get_object_meta headers: {:?}", headers);

	Ok(())
}
