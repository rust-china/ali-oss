use anyhow::Ok;

#[derive(Debug, Clone)]
pub struct BucketStat {
	pub storage: u64,
	pub object_count: u64,
	pub multipart_upload_count: u64,
	pub live_channel_count: u64,
	pub last_modified_time: chrono::DateTime<chrono::Utc>,
	pub standard_storage: u64,
	pub standard_object_count: u64,
	pub infrequent_access_storage: u64,
	pub infrequent_access_object_count: u64,
	pub archive_storage: u64,
	pub archive_real_storage: u64,
	pub archive_object_count: u64,
	pub cold_archive_storage: u64,
	pub cold_archive_real_storage: u64,
	pub cold_archive_object_count: u64,
	pub deep_cold_archive_storage: u64,
	pub deep_cold_archive_real_storage: u64,
	pub deep_cold_archive_object_count: u64,
}

impl BucketStat {
	pub fn new_from_xml_node(node: roxmltree::Node) -> anyhow::Result<Self> {
		let storage = node.descendants().find(|n| n.has_tag_name("Storage")).and_then(|node| node.text()).unwrap_or("").parse()?;
		let object_count = node.descendants().find(|n| n.has_tag_name("ObjectCount")).and_then(|node| node.text()).unwrap_or("").parse()?;
		let multipart_upload_count = node.descendants().find(|n| n.has_tag_name("MultipartUploadCount")).and_then(|node| node.text()).unwrap_or("").parse()?;
		let live_channel_count = node.descendants().find(|n| n.has_tag_name("LiveChannelCount")).and_then(|node| node.text()).unwrap_or("").parse()?;
		let last_modified_time = {
			let seconds: i64 = node.descendants().find(|n| n.has_tag_name("LastModifiedTime")).and_then(|node| node.text()).unwrap_or("").parse()?;
			chrono::DateTime::from_timestamp(seconds, 0).unwrap()
		};
		let standard_storage = node.descendants().find(|n| n.has_tag_name("StandardStorage")).and_then(|node| node.text()).unwrap_or("").parse()?;
		let standard_object_count = node.descendants().find(|n| n.has_tag_name("StandardObjectCount")).and_then(|node| node.text()).unwrap_or("").parse()?;
		let infrequent_access_storage = node.descendants().find(|n| n.has_tag_name("InfrequentAccessStorage")).and_then(|node| node.text()).unwrap_or("").parse()?;
		let infrequent_access_object_count = node.descendants().find(|n| n.has_tag_name("InfrequentAccessObjectCount")).and_then(|node| node.text()).unwrap_or("").parse()?;
		let archive_storage = node.descendants().find(|n| n.has_tag_name("ArchiveStorage")).and_then(|node| node.text()).unwrap_or("").parse()?;
		let archive_real_storage = node.descendants().find(|n| n.has_tag_name("ArchiveRealStorage")).and_then(|node| node.text()).unwrap_or("").parse()?;
		let archive_object_count = node.descendants().find(|n| n.has_tag_name("ArchiveObjectCount")).and_then(|node| node.text()).unwrap_or("").parse()?;
		let cold_archive_storage = node.descendants().find(|n| n.has_tag_name("ColdArchiveStorage")).and_then(|node| node.text()).unwrap_or("").parse()?;
		let cold_archive_real_storage = node.descendants().find(|n| n.has_tag_name("ColdArchiveRealStorage")).and_then(|node| node.text()).unwrap_or("").parse()?;
		let cold_archive_object_count = node.descendants().find(|n| n.has_tag_name("ColdArchiveObjectCount")).and_then(|node| node.text()).unwrap_or("").parse()?;
		let deep_cold_archive_storage = node.descendants().find(|n| n.has_tag_name("DeepColdArchiveStorage")).and_then(|node| node.text()).unwrap_or("").parse()?;
		let deep_cold_archive_real_storage = node.descendants().find(|n| n.has_tag_name("DeepColdArchiveRealStorage")).and_then(|node| node.text()).unwrap_or("").parse()?;
		let deep_cold_archive_object_count = node.descendants().find(|n| n.has_tag_name("DeepColdArchiveObjectCount")).and_then(|node| node.text()).unwrap_or("").parse()?;

		Ok(Self {
			storage,
			object_count,
			multipart_upload_count,
			live_channel_count,
			last_modified_time,
			standard_storage,
			standard_object_count,
			infrequent_access_storage,
			infrequent_access_object_count,
			archive_storage,
			archive_real_storage,
			archive_object_count,
			cold_archive_storage,
			cold_archive_real_storage,
			cold_archive_object_count,
			deep_cold_archive_storage,
			deep_cold_archive_real_storage,
			deep_cold_archive_object_count,
		})
	}
}
