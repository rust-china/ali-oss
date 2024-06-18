use crate::types::OssConfig;
use std::sync::Arc;

#[derive(Debug)]
pub struct Client {
	oss_config: Arc<OssConfig>,
	bucket: crate::Bucket,
}

impl Client {
	pub fn from_env() -> anyhow::Result<Self> {
		let oss_config = Arc::new(OssConfig::from_env()?);
		let bucket = crate::Bucket::new(oss_config.bucket_name.as_str(), oss_config.bucket_location.as_str(), "", None);
		Ok(Self { oss_config, bucket })
	}
	pub fn new<T: ToString>(access_key_id: T, access_key_secret: T, bucket_name: T, bucket_location: T, is_internal: bool) -> Self {
		let oss_config = Arc::new(OssConfig::new(
			access_key_id.to_string(),
			access_key_secret.to_string(),
			bucket_name.to_string().clone(),
			bucket_location.to_string().clone(),
			is_internal,
		));
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
	pub async fn list_objects(&self, prefix: &str) -> anyhow::Result<(Vec<crate::Folder>, Vec<crate::File>)> {
		let mut request = self.oss_config.get_bucket_request(reqwest::Method::GET, None)?;
		request.url_mut().query_pairs_mut().append_pair("list-type", "2").append_pair("delimiter", "/").append_pair("prefix", prefix);
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
	pub async fn list_folders(&self, prefix: &str) -> anyhow::Result<Vec<crate::Folder>> {
		let (folders, _files) = self.list_objects(prefix).await?;
		Ok(folders)
	}
	pub async fn list_files(&self, prefix: &str) -> anyhow::Result<Vec<crate::File>> {
		let (_folders, files) = self.list_objects(prefix).await?;
		Ok(files)
	}

	//https://help.aliyun.com/zh/oss/developer-reference/putobject
	pub async fn put_object<T: Into<bytes::Bytes>>(&self, object_name: &str, bytes: T) -> anyhow::Result<reqwest::header::HeaderMap> {
		let mut request = self.oss_config.get_bucket_request(reqwest::Method::PUT, Some(bytes.into()))?;
		request.url_mut().set_path(object_name);
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
		let mut request = self.oss_config.get_bucket_request(reqwest::Method::PUT, None)?;
		request.url_mut().set_path(object_name);
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
		let mut request = self.oss_config.get_bucket_request(reqwest::Method::GET, None)?;
		request.url_mut().set_path(object_name);
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
		let mut request = self.oss_config.get_bucket_request(reqwest::Method::DELETE, None)?;
		request.url_mut().set_path(object_name);
		self.oss_config.sign_header_request(&mut request)?;

		let response = self.oss_config.get_request_builder(request)?.send().await?;
		if !response.status().is_success() {
			return Err(anyhow::anyhow!(response.text().await?));
		}
		Ok(())
	}
}
