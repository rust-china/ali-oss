#[derive(Debug, Clone)]
pub struct File {
	pub name: String,
	pub r#type: String,
	pub size: u64,
	pub etag: String,
	pub last_modified: chrono::DateTime<chrono::Utc>,
	pub storage_class: String,
}

impl File {
	pub fn new<T: ToString>(name: T, r#type: T, size: u64, etag: T, last_modified: chrono::DateTime<chrono::Utc>, storage_class: T) -> Self {
		Self {
			name: name.to_string(),
			r#type: r#type.to_string(),
			size,
			etag: etag.to_string(),
			last_modified,
			storage_class: storage_class.to_string(),
		}
	}
}

impl File {
	pub fn new_from_xml_node(node: roxmltree::Node) -> anyhow::Result<Self> {
		let name = node.descendants().find(|n| n.has_tag_name("Key")).and_then(|node| node.text()).unwrap_or("");
		let r#type = node.descendants().find(|n| n.has_tag_name("Type")).and_then(|node| node.text()).unwrap_or("");
		let size = node.descendants().find(|n| n.has_tag_name("Size")).and_then(|node| node.text()).unwrap_or("").parse()?;
		let etag = node.descendants().find(|n| n.has_tag_name("ETag")).and_then(|node| node.text()).unwrap_or("").trim_matches('"');
		let last_modified = node.descendants().find(|n| n.has_tag_name("LastModified")).and_then(|node| node.text()).unwrap_or("");
		let storage_class = node.descendants().find(|n| n.has_tag_name("StorageClass")).and_then(|node| node.text()).unwrap_or("");
		Ok(Self::new(name, r#type, size, etag, last_modified.parse()?, storage_class))
	}
}
