// examples/common/mod.rs
pub mod common;
use ali_oss::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	common::load_env()?;

	let oss_client = crate::Client::from_env()?;
	let target_object_name = oss_client.get_symlink("latest/lib_symlink.rs").await?;
	println!("target_object_name: {:?}", target_object_name);

	Ok(())
}
