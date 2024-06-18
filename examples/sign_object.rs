// examples/common/mod.rs
pub mod common;
use ali_oss::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	common::load_env()?;

	let oss_client = crate::Client::from_env()?;
	let url = oss_client.sign_object("1.html", std::time::Duration::from_secs(300)).await?;
	println!("sign_object url: {:?}", url);

	Ok(())
}
