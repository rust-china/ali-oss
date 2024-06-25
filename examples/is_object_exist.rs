// examples/common/mod.rs
pub mod common;
use ali_oss::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	common::load_env()?;

	let oss_client = crate::Client::from_env()?;
	let is_exist = oss_client.is_object_exist("latest/lib_symlink.rs").await?;
	println!("is_exist: {}", is_exist);

	Ok(())
}
