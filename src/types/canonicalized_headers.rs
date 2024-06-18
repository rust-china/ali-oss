use std::collections::BTreeMap;
#[derive(Debug)]
pub struct CanonicalizedHeaders(Option<BTreeMap<String, String>>);

impl Default for CanonicalizedHeaders {
	fn default() -> Self {
		Self(None)
	}
}

impl CanonicalizedHeaders {
	pub fn new(map: Option<BTreeMap<String, String>>) -> Self {
		Self(map)
	}
	pub fn string_for_sign(&self) -> Option<String> {
		if let Some(map) = &self.0 {
			let mut buf = String::new();
			for (k, v) in map.iter() {
				buf.push_str(k);
				buf.push(':');
				buf.push_str(v);
				buf.push('\n');
			}
			Some(buf)
		} else {
			None
		}
	}
}

impl From<&reqwest::Request> for CanonicalizedHeaders {
	fn from(request: &reqwest::Request) -> Self {
		let mut headers = BTreeMap::new();
		for (k, v) in request.headers() {
			let key = k.as_str();
			if key.starts_with("x-oss-") {
				match v.to_str() {
					Ok(v) => {
						headers.insert(key.to_string(), v.to_string());
					}
					Err(_) => {}
				}
			}
		}
		Self::new(if headers.is_empty() { None } else { Some(headers) })
	}
}
