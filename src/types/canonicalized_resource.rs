pub struct CanonicalizedResource(String);

impl Default for CanonicalizedResource {
	fn default() -> Self {
		Self("/".to_string())
	}
}

impl CanonicalizedResource {
	pub fn new<T: ToString>(resource: T) -> Self {
		Self(resource.to_string())
	}
	pub fn as_str(&self) -> &str {
		&self.0
	}
}
