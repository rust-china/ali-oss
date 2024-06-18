#[derive(Debug, Clone)]
pub struct Folder {
	pub name: String,
}

impl Folder {
	pub fn new<T: ToString>(name: T) -> Self {
		Self { name: name.to_string() }
	}
}

impl Folder {
	pub fn new_from_xml_node(node: roxmltree::Node) -> anyhow::Result<Self> {
		let name = node.descendants().find(|n| n.has_tag_name("Prefix")).and_then(|node| node.text()).unwrap_or("");
		Ok(Self { name: name.to_string() })
	}
}
