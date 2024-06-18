use ali_oss::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	let env_path = std::env::current_dir()?.join("examples/.env");
	dotenvy::from_path(env_path)?;
	let env_local_path = std::env::current_dir()?.join("examples/.env.local");
	let _ = dotenvy::from_path_override(env_local_path);

	let oss_client = crate::Client::from_env()?;
	let bucket_stat = oss_client.get_bucket_stat().await?;
	println!("bucket_stat: {:?}", bucket_stat);

	Ok(())
}
