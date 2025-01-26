use crate::types;
use base64::{engine::general_purpose::STANDARD, Engine};
use reqwest::Method;
use std::ops::Deref;

#[derive(Debug)]
pub struct HeaderSignature {
	pub method: Method,
	pub content_md5: Option<String>,
	pub content_type: Option<String>,
	pub date: chrono::DateTime<chrono::Utc>,
	pub canonicalized_oss_headers: types::CanonicalizedHeaders,
	pub canonicalized_resource: types::CanonicalizedResource,
}

impl Default for HeaderSignature {
	fn default() -> Self {
		Self {
			method: Method::GET,
			content_md5: None,
			content_type: None,
			date: chrono::Utc::now(),
			canonicalized_oss_headers: types::CanonicalizedHeaders::default(),
			canonicalized_resource: types::CanonicalizedResource::default(),
		}
	}
}

impl HeaderSignature {
	pub fn new(method: Method, content_md5: Option<String>, content_type: Option<String>, date: chrono::DateTime<chrono::Utc>, canonicalized_oss_headers: types::CanonicalizedHeaders, canonicalized_resource: types::CanonicalizedResource) -> Self {
		Self {
			method,
			content_md5,
			content_type,
			date,
			canonicalized_oss_headers,
			canonicalized_resource,
		}
	}
	pub fn get_date_string(&self) -> String {
		self.date.format("%a, %d %b %Y %T GMT").to_string()
	}
	pub fn get_string_to_sign(&self) -> String {
		let verb = self.method.as_str();
		let date = self.get_date_string();
		let canonicalized_resource = self.canonicalized_resource.as_str();
		let string_to_sign = {
			let mut buf = String::new();
			buf.push_str(verb);
			buf.push('\n');
			if let Some(content_md5) = &self.content_md5 {
				buf.push_str(content_md5);
			}
			buf.push('\n');
			if let Some(content_type) = &self.content_type {
				buf.push_str(content_type);
			}
			buf.push('\n');
			buf.push_str(&date);
			buf.push('\n');
			if let Some(value) = self.canonicalized_oss_headers.string_for_sign() {
				buf.push_str(value.as_str());
			}
			buf.push_str(canonicalized_resource);
			buf
		};
		string_to_sign
	}
}

impl super::SignatureAble for HeaderSignature {
	fn get_signature_string<T: Deref<Target = crate::OssConfig>>(&self, oss_config: T) -> String {
		let string_to_sign = self.get_string_to_sign();
		let key = ring::hmac::Key::new(ring::hmac::HMAC_SHA1_FOR_LEGACY_USE_ONLY, oss_config.access_key_secret.as_bytes());
		let hmac_signature = ring::hmac::sign(&key, string_to_sign.as_bytes());
		STANDARD.encode(hmac_signature.as_ref())
	}
}
