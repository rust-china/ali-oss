// examples/common/mod.rs
pub mod common;
use ali_oss::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	common::load_env()?;

	let oss_client = crate::Client::from_env()?;
	let response = reqwest::get("https://debug.zezeping.com/1.html").await?;
	let headers = oss_client.put_object_stream("/1.html", response.bytes_stream()).await?;
	println!("put_object headers: {:?}", headers);

	Ok(())
}
