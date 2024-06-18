use std::sync::Mutex;
#[derive(Debug)]
pub struct Bucket {
	pub name: String,
	pub location: crate::types::BucketLocation,
	pub comment: String,
	pub creation_date: Mutex<Option<chrono::DateTime<chrono::Utc>>>,
}

impl Bucket {
	pub fn new<T: ToString, U: ToString>(name: T, location: T, comment: U, creation_date: Option<chrono::DateTime<chrono::Utc>>) -> Self {
		Self {
			name: name.to_string(),
			location: crate::types::BucketLocation::new(location),
			comment: comment.to_string(),
			creation_date: Mutex::new(creation_date),
		}
	}
	pub fn new_from_xml_node(node: roxmltree::Node) -> anyhow::Result<Self> {
		let name = node.descendants().find(|n| n.has_tag_name("Name")).and_then(|node| node.text()).unwrap_or("");
		let location = node.descendants().find(|n| n.has_tag_name("Location")).and_then(|node| node.text()).unwrap_or("");
		let creation_date = node.descendants().find(|n| n.has_tag_name("CreationDate")).and_then(|node| node.text()).unwrap_or("");
		let comment = node.descendants().find(|n| n.has_tag_name("Comment")).and_then(|node| node.text()).unwrap_or("");
		Ok(Self::new(name, location, comment, Some(creation_date.parse()?)))
	}
}
