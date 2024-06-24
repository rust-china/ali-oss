use crate::types::OssConfig;
use crate::SignatureAble;

#[derive(Debug)]
pub struct Client {
	oss_config: OssConfig,
	bucket: crate::Bucket,
}

impl Client {
	pub fn from_env() -> anyhow::Result<Self> {
		let oss_config = OssConfig::from_env()?;
		let bucket = crate::Bucket::new(oss_config.bucket_name.as_str(), oss_config.bucket_location.as_str(), "", None);
		Ok(Self { oss_config, bucket })
	}
	pub fn new<T: ToString>(access_key_id: T, access_key_secret: T, bucket_name: T, bucket_location: T, path: T, is_internal: bool) -> Self {
		let oss_config = OssConfig::new(
			access_key_id.to_string(),
			access_key_secret.to_string(),
			bucket_name.to_string().clone(),
			bucket_location.to_string().clone(),
			path.to_string().clone(),
			is_internal,
		);
		let bucket = crate::Bucket::new(bucket_name, bucket_location, "", None);
		Self { oss_config, bucket }
	}
}

impl Client {
	// https://www.alibabacloud.com/help/zh/oss/developer-reference/listbuckets
	pub async fn list_buckets(&self) -> anyhow::Result<Vec<crate::Bucket>> {
		let mut request = self.oss_config.get_endpoint_request(reqwest::Method::GET)?;
		self.oss_config.sign_header_request(&mut request)?;

		let response = self.oss_config.get_request_builder(request)?.send().await?;
		if !response.status().is_success() {
			return Err(anyhow::anyhow!(response.text().await?));
		}
		let xml_data = response.text().await?;
		let doc: roxmltree::Document = roxmltree::Document::parse(&xml_data)?;
		let mut buckets = Vec::new();
		let buckets_node = doc.descendants().find(|n| n.has_tag_name("Buckets")).ok_or_else(|| anyhow::anyhow!("Buckets node not found"))?;
		for bucket_node in buckets_node.descendants().filter(|n| n.has_tag_name("Bucket")) {
			buckets.push(crate::Bucket::new_from_xml_node(bucket_node)?);
		}
		Ok(buckets)
	}

	// https://help.aliyun.com/zh/oss/developer-reference/putbucket
	pub async fn put_bucket(&self) -> anyhow::Result<crate::Bucket> {
		let mut request = self.oss_config.get_bucket_request(reqwest::Method::PUT, None)?;
		self.oss_config.sign_header_request(&mut request)?;

		let response = self.oss_config.get_request_builder(request)?.send().await?;
		if !response.status().is_success() {
			return Err(anyhow::anyhow!(response.text().await?));
		}
		let creation_date = {
			let date = response.headers().get("date");
			if let Some(date) = date {
				Some(chrono::DateTime::parse_from_rfc2822(date.to_str()?)?.into())
			} else {
				None
			}
		};
		*self.bucket.creation_date.lock().unwrap() = creation_date.clone();
		Ok(crate::Bucket::new(self.bucket.name.as_str(), self.bucket.location.as_str(), "", creation_date))
	}

	// https://help.aliyun.com/zh/oss/developer-reference/getbucketinfo
	pub async fn get_bucket_info(&self) -> anyhow::Result<Option<crate::Bucket>> {
		static BUCKET_INFO: &str = "bucketInfo";
		let mut request = self.oss_config.get_bucket_request(reqwest::Method::GET, None)?;
		request.url_mut().set_query(Some(BUCKET_INFO));
		self.oss_config.sign_header_request(&mut request)?;

		let response = self.oss_config.get_request_builder(request)?.send().await?;
		if !response.status().is_success() {
			return Err(anyhow::anyhow!(response.text().await?));
		}
		let xml_data = response.text().await?;
		let doc: roxmltree::Document = roxmltree::Document::parse(&xml_data)?;
		if let Some(bucket_node) = doc.descendants().find(|n| n.has_tag_name("Bucket")) {
			let bucket = crate::Bucket::new_from_xml_node(bucket_node)?;
			*self.bucket.creation_date.lock().unwrap() = bucket.creation_date.lock().unwrap().clone();
			return Ok(Some(bucket));
		}
		Ok(None)
	}

	// https://www.alibabacloud.com/help/zh/oss/developer-reference/getbucketlocation
	pub async fn get_bucket_location(&self) -> anyhow::Result<crate::types::BucketLocation> {
		static BUCKET_LOCATION: &str = "location";
		let mut request = self.oss_config.get_bucket_request(reqwest::Method::GET, None)?;
		request.url_mut().set_query(Some(BUCKET_LOCATION));
		self.oss_config.sign_header_request(&mut request)?;

		let response = self.oss_config.get_request_builder(request)?.send().await?;
		if !response.status().is_success() {
			return Err(anyhow::anyhow!(response.text().await?));
		}
		let xml_data = response.text().await?;
		let doc: roxmltree::Document = roxmltree::Document::parse(&xml_data)?;
		crate::types::BucketLocation::new_from_xml_node(doc.root())
	}

	// https://help.aliyun.com/zh/oss/developer-reference/getbucketstat
	pub async fn get_bucket_stat(&self) -> anyhow::Result<crate::types::BucketStat> {
		static BUCKET_STAT: &str = "stat";
		let mut request = self.oss_config.get_bucket_request(reqwest::Method::GET, None)?;
		request.url_mut().set_query(Some(BUCKET_STAT));
		self.oss_config.sign_header_request(&mut request)?;

		let response = self.oss_config.get_request_builder(request)?.send().await?;
		if !response.status().is_success() {
			return Err(anyhow::anyhow!(response.text().await?));
		}
		let xml_data = response.text().await?;
		let doc: roxmltree::Document = roxmltree::Document::parse(&xml_data)?;
		crate::types::BucketStat::new_from_xml_node(doc.root())
	}

	// https://help.aliyun.com/zh/oss/developer-reference/deletebucket
	pub async fn delete_bucket(&self) -> anyhow::Result<()> {
		let mut request = self.oss_config.get_bucket_request(reqwest::Method::DELETE, None)?;
		self.oss_config.sign_header_request(&mut request)?;

		let response = self.oss_config.get_request_builder(request)?.send().await?;
		if !response.status().is_success() {
			return Err(anyhow::anyhow!(response.text().await?));
		}
		*self.bucket.creation_date.lock().unwrap() = None;
		Ok(())
	}
}

impl Client {
	// https://www.alibabacloud.com/help/zh/oss/developer-reference/listobjectsv2
	pub async fn list_objects(&self, prefix: Option<&str>, delimiter: Option<&str>) -> anyhow::Result<(Vec<crate::Folder>, Vec<crate::File>)> {
		let mut request = self.oss_config.get_bucket_request(reqwest::Method::GET, None)?;
		request.url_mut().query_pairs_mut().append_pair("list-type", "2");
		if let Some(prefix) = prefix {
			let prefix = self.oss_config.get_object_name(prefix);
			request.url_mut().query_pairs_mut().append_pair("prefix", prefix.as_ref());
		}
		if let Some(delimiter) = delimiter {
			request.url_mut().query_pairs_mut().append_pair("delimiter", delimiter);
		}
		self.oss_config.sign_header_request(&mut request)?;

		let response = self.oss_config.get_request_builder(request)?.send().await?;
		if !response.status().is_success() {
			return Err(anyhow::anyhow!(response.text().await?));
		}
		let xml_data = response.text().await?;
		// println!("xml_data: {}", xml_data);
		let doc: roxmltree::Document = roxmltree::Document::parse(&xml_data)?;
		let mut folders = Vec::new();
		for folder_node in doc.descendants().filter(|n| n.has_tag_name("CommonPrefixes")) {
			folders.push(crate::Folder::new_from_xml_node(folder_node)?);
		}
		let mut files = Vec::new();
		for file_node in doc.descendants().filter(|n| n.has_tag_name("Contents")) {
			files.push(crate::File::new_from_xml_node(file_node)?);
		}
		Ok((folders, files))
	}
	pub async fn list_folders(&self, prefix: Option<&str>) -> anyhow::Result<Vec<crate::Folder>> {
		let (folders, _files) = self.list_objects(prefix, Some("/")).await?;
		Ok(folders)
	}
	pub async fn list_files(&self, prefix: Option<&str>) -> anyhow::Result<Vec<crate::File>> {
		let (_folders, files) = self.list_objects(prefix, Some("/")).await?;
		Ok(files)
	}

	//https://help.aliyun.com/zh/oss/developer-reference/putobject
	pub async fn put_object<T: Into<bytes::Bytes>>(&self, object_name: &str, bytes: T) -> anyhow::Result<reqwest::header::HeaderMap> {
		let object_name = self.oss_config.get_object_name(object_name);
		let mut request = self.oss_config.get_bucket_request(reqwest::Method::PUT, Some(bytes.into()))?;
		request.url_mut().set_path(object_name.as_ref());
		self.oss_config.sign_header_request(&mut request)?;

		let response = self.oss_config.get_request_builder(request)?.send().await?;
		if !response.status().is_success() {
			return Err(anyhow::anyhow!(response.text().await?));
		}
		Ok(response.headers().clone())
	}
	pub async fn put_object_stream<S>(&self, object_name: &str, stream: S) -> anyhow::Result<reqwest::header::HeaderMap>
	where
		S: futures::stream::Stream<Item = reqwest::Result<bytes::Bytes>> + Send + Sync + 'static,
	{
		let object_name = self.oss_config.get_object_name(object_name);
		let mut request = self.oss_config.get_bucket_request(reqwest::Method::PUT, None)?;
		request.url_mut().set_path(object_name.as_ref());
		*request.body_mut() = Some(reqwest::Body::wrap_stream(stream));
		self.oss_config.sign_header_request(&mut request)?;

		let response = self.oss_config.get_request_builder(request)?.send().await?;
		if !response.status().is_success() {
			return Err(anyhow::anyhow!(response.text().await?));
		}
		Ok(response.headers().clone())
	}

	// https://help.aliyun.com/zh/oss/developer-reference/getobject
	pub async fn get_object(&self, object_name: &str) -> anyhow::Result<(bytes::Bytes, reqwest::header::HeaderMap)> {
		let object_name = self.oss_config.get_object_name(object_name);
		let mut request = self.oss_config.get_bucket_request(reqwest::Method::GET, None)?;
		request.url_mut().set_path(object_name.as_ref());
		self.oss_config.sign_header_request(&mut request)?;

		let response = self.oss_config.get_request_builder(request)?.send().await?;
		if !response.status().is_success() {
			return Err(anyhow::anyhow!(response.text().await?));
		}
		let headers = response.headers().clone();
		Ok((response.bytes().await?, headers))
	}

	// https://help.aliyun.com/zh/oss/developer-reference/deleteobject
	pub async fn delete_object(&self, object_name: &str) -> anyhow::Result<()> {
		let object_name = self.oss_config.get_object_name(object_name);
		let mut request = self.oss_config.get_bucket_request(reqwest::Method::DELETE, None)?;
		request.url_mut().set_path(object_name.as_ref());
		self.oss_config.sign_header_request(&mut request)?;

		let response = self.oss_config.get_request_builder(request)?.send().await?;
		if !response.status().is_success() {
			return Err(anyhow::anyhow!(response.text().await?));
		}
		Ok(())
	}

	// https://help.aliyun.com/zh/oss/developer-reference/copyobject
	pub async fn copy_object(&self, dest_object_name: &str, source_object_name: &str) -> anyhow::Result<reqwest::header::HeaderMap> {
		let dest_object_name = self.oss_config.get_object_name(dest_object_name);
		let source_object_name = self.oss_config.get_object_name(source_object_name);
		let mut request = self.oss_config.get_bucket_request(reqwest::Method::PUT, None)?;
		request.url_mut().set_path(dest_object_name.as_ref());
		request.headers_mut().insert("x-oss-copy-source", format!("/{}/{}", self.oss_config.bucket_name, source_object_name).try_into()?);
		self.oss_config.sign_header_request(&mut request)?;

		let response = self.oss_config.get_request_builder(request)?.send().await?;
		if !response.status().is_success() {
			return Err(anyhow::anyhow!(response.text().await?));
		}
		Ok(response.headers().clone())
	}

	// https://help.aliyun.com/zh/oss/developer-reference/appendobject
	pub async fn append_object<T: Into<bytes::Bytes>>(&self, object_name: &str, bytes: T, position: usize) -> anyhow::Result<reqwest::header::HeaderMap> {
		let object_name = self.oss_config.get_object_name(object_name);
		static APPEND: &str = "append";
		let mut request = self.oss_config.get_bucket_request(reqwest::Method::POST, Some(bytes.into()))?;
		request.url_mut().set_path(object_name.as_ref());
		request.headers_mut().insert("position", position.try_into()?);
		request.url_mut().set_query(Some(APPEND));
		request.url_mut().query_pairs_mut().append_pair("position", position.to_string().as_str());
		self.oss_config.sign_header_request(&mut request)?;

		let response = self.oss_config.get_request_builder(request)?.send().await?;
		if !response.status().is_success() {
			return Err(anyhow::anyhow!(response.text().await?));
		}
		Ok(response.headers().clone())
	}

	// https://help.aliyun.com/zh/oss/developer-reference/headobject
	pub async fn head_object(&self, object_name: &str) -> anyhow::Result<reqwest::header::HeaderMap> {
		let object_name = self.oss_config.get_object_name(object_name);
		let mut request = self.oss_config.get_bucket_request(reqwest::Method::HEAD, None)?;
		request.url_mut().set_path(object_name.as_ref());
		self.oss_config.sign_header_request(&mut request)?;

		let response = self.oss_config.get_request_builder(request)?.send().await?;
		if !response.status().is_success() {
			return Err(anyhow::anyhow!(response.text().await?));
		}
		Ok(response.headers().clone())
	}

	// https://help.aliyun.com/zh/oss/developer-reference/getobjectmeta
	pub async fn get_object_meta(&self, object_name: &str) -> anyhow::Result<reqwest::header::HeaderMap> {
		let object_name = self.oss_config.get_object_name(object_name);
		static OBJECT_META: &str = "objectMeta";
		let mut request = self.oss_config.get_bucket_request(reqwest::Method::HEAD, None)?;
		request.url_mut().set_path(object_name.as_ref());
		request.url_mut().set_query(Some(OBJECT_META));
		self.oss_config.sign_header_request(&mut request)?;

		let response = self.oss_config.get_request_builder(request)?.send().await?;
		if !response.status().is_success() {
			return Err(anyhow::anyhow!(response.text().await?));
		}
		Ok(response.headers().clone())
	}

	// https://www.alibabacloud.com/help/zh/oss/developer-reference/ddd-signatures-to-urls
	pub async fn sign_object(&self, object_name: &str, expires_duration: std::time::Duration) -> anyhow::Result<String> {
		let object_name = self.oss_config.get_object_name(object_name);
		let expires_time = {
			let datetime: chrono::DateTime<chrono::Utc> = std::time::SystemTime::now().into();
			let expires_time = datetime + chrono::Duration::from_std(expires_duration)?;
			expires_time
		};
		let mut object_url = {
			let host = format!("{}.{}.aliyuncs.com", self.oss_config.bucket_name, self.oss_config.bucket_location.as_str());
			let object_link = format!("https://{}/{}", host, object_name);
			reqwest::Url::parse(&object_link)?
		};
		let signature_string = crate::types::ParamSignature::new(
			reqwest::Method::GET,
			None,
			None,
			expires_time.clone(),
			crate::types::CanonicalizedHeaders::new(None),
			crate::types::CanonicalizedResource::new(format!("/{}/{}", self.oss_config.bucket_name, object_name)),
		)
		.get_signature_string(&self.oss_config);
		object_url
			.query_pairs_mut()
			.append_pair("OSSAccessKeyId", &self.oss_config.access_key_id)
			.append_pair("Expires", expires_time.timestamp().to_string().as_str())
			.append_pair("Signature", &signature_string);

		Ok(object_url.to_string())
	}
}

impl Client {
	// https://www.alibabacloud.com/help/zh/oss/developer-reference/putsymlink
	pub async fn put_symlink(&self, symlink_object_name: &str, target_object_name: &str) -> anyhow::Result<()> {
		static SYMLINK: &str = "symlink";
		let symlink_object_name = self.oss_config.get_object_name(symlink_object_name);
		let target_object_name = self.oss_config.get_object_name(target_object_name);
		let mut request = self.oss_config.get_bucket_request(reqwest::Method::PUT, None)?;
		request.url_mut().set_path(symlink_object_name.as_ref());
		request.headers_mut().insert("x-oss-symlink-target", target_object_name.as_ref().try_into()?);
		request.url_mut().set_query(Some(SYMLINK));
		self.oss_config.sign_header_request(&mut request)?;

		let response = self.oss_config.get_request_builder(request)?.send().await?;
		if !response.status().is_success() {
			return Err(anyhow::anyhow!(response.text().await?));
		}
		Ok(())
	}

	// https://www.alibabacloud.com/help/zh/oss/developer-reference/getsymlink
	pub async fn get_symlink(&self, object_name: &str) -> anyhow::Result<String> {
		let object_name = self.oss_config.get_object_name(object_name);
		static SYMLINK: &str = "symlink";
		let mut request = self.oss_config.get_bucket_request(reqwest::Method::GET, None)?;
		request.url_mut().set_path(object_name.as_ref());
		request.url_mut().set_query(Some(SYMLINK));
		self.oss_config.sign_header_request(&mut request)?;

		let response = self.oss_config.get_request_builder(request)?.send().await?;
		if !response.status().is_success() {
			return Err(anyhow::anyhow!(response.text().await?));
		}
		Ok(response.headers().get("x-oss-symlink-target").ok_or(anyhow::anyhow!("no symlink target"))?.to_str()?.to_owned())
	}
}
