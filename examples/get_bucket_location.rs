use ali_oss::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	let env_path = std::env::current_dir()?.join("examples/.env");
	dotenvy::from_path(env_path)?;
	let env_local_path = std::env::current_dir()?.join("examples/.env.local");
	let _ = dotenvy::from_path_override(env_local_path);

	let oss_client = crate::Client::from_env()?;
	let bucket_location = oss_client.get_bucket_location().await?;
	println!("bucket_location: {:?}", bucket_location);

	Ok(())
}
