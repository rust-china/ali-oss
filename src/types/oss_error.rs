#[derive(Debug)]
pub struct OssError {
	code: String,
	message: String,
	xml_string: String,
}

impl OssError {
	pub fn new_from_xml_string(xml_string: String) -> anyhow::Result<Self> {
		let doc: roxmltree::Document = roxmltree::Document::parse(&xml_string)?;
		Ok(Self {
			code: "".to_string(),
			message: "".to_string(),
			xml_string,
		})
	}
}
