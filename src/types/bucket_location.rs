#[derive(Debug, Clone)]
pub struct BucketLocation(String);

impl AsRef<str> for BucketLocation {
	fn as_ref(&self) -> &str {
		&self.0
	}
}

impl BucketLocation {
	pub fn new<T: ToString>(location: T) -> Self {
		Self(location.to_string())
	}
	pub fn new_from_xml_node(node: roxmltree::Node) -> anyhow::Result<Self> {
		let location = node.descendants().find(|n| n.has_tag_name("LocationConstraint")).and_then(|node| node.text()).unwrap_or("");
		Ok(Self(location.to_string()))
	}
	pub fn as_str(&self) -> &str {
		&self.0
	}
}
