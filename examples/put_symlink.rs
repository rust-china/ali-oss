// examples/common/mod.rs
pub mod common;
use ali_oss::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	common::load_env()?;

	let oss_client = crate::Client::from_env()?;
	oss_client.put_symlink("/latest/lib_symlink.rs", "lib.rs").await?;
	println!("put_symlink success");

	Ok(())
}
