// examples/common/mod.rs
pub mod common;
use ali_oss::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	common::load_env()?;

	let oss_client = crate::Client::from_env()?;

	let file_path = std::env::current_dir()?.join("src/lib.rs");
	let buffer = std::fs::read(file_path)?;
	let headers = oss_client.append_object("lib_twice.rs", buffer.clone(), 0).await?;
	println!("append_object1 headers: {:?}", headers);

	let next_position: usize = headers.get("x-oss-next-append-position").unwrap().to_str()?.parse().unwrap();
	let headers = oss_client.append_object("lib_twice.rs", buffer, next_position).await?;
	println!("append_object2 headers: {:?}", headers);

	Ok(())
}
