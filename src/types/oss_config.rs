use crate::SignatureAble;
use reqwest::{header, Method, Url};

#[derive(Debug, Clone)]
pub struct OssConfig {
	pub access_key_id: String,
	pub access_key_secret: String,
	pub bucket_name: String,
	pub bucket_location: crate::types::BucketLocation,
	pub is_internal: bool,
}

impl OssConfig {
	pub fn from_env() -> anyhow::Result<Self> {
		let access_key_id = std::env::var("ALI_OSS_ACCESS_KEY_ID")?;
		let access_key_secret = std::env::var("ALI_OSS_ACCESS_KEY_SECRET")?;
		let bucket_name = std::env::var("ALI_OSS_BUCKET")?;
		let bucket_location = std::env::var("ALI_OSS_LOCATION")?;
		let internal = std::env::var("ALI_OSS_INTERNAL")?;

		// let bucket = crate::Bucket::new(bucket_name, bucket_location, "".to_owned(), None);
		let client = Self::new(access_key_id, access_key_secret, bucket_name, bucket_location, internal == "true");
		Ok(client)
	}
	pub fn new(access_key_id: String, access_key_secret: String, bucket_name: String, bucket_location: String, is_internal: bool) -> Self {
		Self {
			access_key_id: access_key_id.to_string(),
			access_key_secret: access_key_secret.to_string(),
			bucket_name: bucket_name.to_string(),
			bucket_location: crate::types::BucketLocation::new(bucket_location),
			is_internal,
		}
	}
	/// # 返回 endpoint 对应的链接地址
	/// 可以是内网地址，默认为外网地址
	/// ```
	/// # use ali_oss::OssConfig;
	/// # use reqwest::Url;
	/// assert_eq!(OssConfig::generate_endpoint_url("oss-cn-hangzhou", true).unwrap(), Url::parse("https://oss-cn-hangzhou-internal.aliyuncs.com").unwrap());
	/// assert_eq!(OssConfig::generate_endpoint_url("oss-cn-hangzhou", false).unwrap(), Url::parse("https://oss-cn-hangzhou.aliyuncs.com").unwrap());
	/// ```
	pub fn generate_endpoint_url(bucket_location: &str, is_internal: bool) -> anyhow::Result<Url> {
		let src = format!("https://{}{}.aliyuncs.com", bucket_location, if is_internal { "-internal" } else { "" });
		Ok(Url::parse(&src)?)
	}
	/// # 返回 bucket 对应的链接地址
	/// 可以是内网地址，默认为外网地址
	/// ```
	/// # use ali_oss::OssConfig;
	/// # use reqwest::Url;
	/// assert_eq!(OssConfig::generate_bucket_url("hello", "oss-cn-hangzhou", true).unwrap(), Url::parse("https://hello.oss-cn-hangzhou-internal.aliyuncs.com").unwrap());
	/// assert_eq!(OssConfig::generate_bucket_url("hello", "oss-cn-hangzhou", false).unwrap(), Url::parse("https://hello.oss-cn-hangzhou.aliyuncs.com").unwrap());
	/// ```
	pub fn generate_bucket_url(backet_name: &str, bucket_location: &str, is_internal: bool) -> anyhow::Result<Url> {
		let src = format!("https://{}.{}{}.aliyuncs.com", backet_name, bucket_location, if is_internal { "-internal" } else { "" });
		Ok(Url::parse(&src)?)
	}
}

impl OssConfig {
	pub fn get_endpoint_url(&self) -> anyhow::Result<Url> {
		Self::generate_endpoint_url(&self.bucket_location.as_str(), self.is_internal)
	}

	pub fn get_endpoint_request(&self, method: Method) -> anyhow::Result<reqwest::Request> {
		let url = self.get_endpoint_url()?;
		let request = reqwest::Request::new(method, url);
		// request.headers_mut().insert(CONTENT_TYPE, "application/xml".try_into()?);
		Ok(request)
	}

	pub fn get_bucket_url(&self) -> anyhow::Result<Url> {
		Self::generate_bucket_url(&self.bucket_name, &self.bucket_location.as_str(), self.is_internal)
	}

	pub fn get_bucket_request(&self, method: Method, body: Option<bytes::Bytes>) -> anyhow::Result<reqwest::Request> {
		let url = self.get_bucket_url()?;
		let mut request = reqwest::Request::new(method, url);
		if let Some(body) = body {
			request.headers_mut().insert(header::CONTENT_LENGTH, body.len().try_into()?);
			*request.body_mut() = Some(reqwest::Body::from(body));
		}
		// request.headers_mut().insert(header::CONTENT_TYPE, "text/plain".try_into()?);
		Ok(request)
	}

	pub(crate) fn sign_header_request(&self, request: &mut reqwest::Request) -> anyhow::Result<()> {
		let content_md5 = {
			let content_md5 = request.headers().get("Content-MD5");
			if let Some(content_md5) = content_md5 {
				Some(content_md5.to_str()?.to_owned())
			} else {
				None
			}
		};
		let content_type = {
			let content_type = request.headers().get("Content-Type");
			if let Some(content_type) = content_type {
				Some(content_type.to_str()?.to_owned())
			} else {
				None
			}
		};
		let canonicalized_oss_headers: crate::types::CanonicalizedHeaders = (&*request).into();
		let canonicalized_resource = {
			let host = request.url().host_str().ok_or(anyhow::anyhow!("host not found"))?;
			if host.starts_with(self.bucket_location.as_str()) {
				crate::types::CanonicalizedResource::default()
			} else {
				let path = request.url().path();
				let query = request.url().query();
				let mut resource = format!("/{}{}", self.bucket_name, path);
				if let Some(query) = query {
					if let Some(first_query) = query.split('&').next() {
						if !first_query.contains("=") {
							resource.push_str(&format!("?{}", query));
						}
					}
				}
				crate::types::CanonicalizedResource::new(resource)
			}
		};
		let header_signature = crate::types::HeaderSignature::new(request.method().clone(), content_md5, content_type, chrono::Utc::now(), canonicalized_oss_headers, canonicalized_resource);
		let signatured_string = header_signature.get_signature_string(self);
		let authorization = format!("OSS {}:{}", self.access_key_id, signatured_string);
		request.headers_mut().insert("Authorization", authorization.try_into()?);
		request.headers_mut().insert("Date", header_signature.get_date_string().try_into()?);
		Ok(())
	}
	pub(crate) fn get_request_builder(&self, request: reqwest::Request) -> anyhow::Result<reqwest::RequestBuilder> {
		let req_client = reqwest::Client::new();
		Ok(reqwest::RequestBuilder::from_parts(req_client, request))
	}
}
