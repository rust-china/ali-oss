pub fn load_env() -> anyhow::Result<()> {
	let env_path = std::env::current_dir()?.join("examples/common/.env");
	dotenvy::from_path(env_path)?;
	let env_local_path = std::env::current_dir()?.join("examples/common/.env.local");
	let _ = dotenvy::from_path_override(env_local_path);
	Ok(())
}
