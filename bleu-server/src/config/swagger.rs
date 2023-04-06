use paperclip::v2::models::{DefaultApiRaw, Info, Tag};

pub fn load() -> DefaultApiRaw {
	let mut spec = DefaultApiRaw::default();
	spec.tags = simple_tags(vec!["EthereumBlock", "EthereumTransaction", "EthereumLog"]);
	spec.info = simple_info("0.1", "Ethereum Explorer");
	spec
}

fn simple_tags(tag_names: Vec<&str>) -> Vec<Tag> {
	let mut tags = Vec::new();
	for tag_name in tag_names.into_iter() {
		let tag = Tag { name: tag_name.to_string(), description: None, external_docs: None };
		tags.push(tag);
	}
	tags
}

fn simple_info(version: &str, title: &str) -> Info {
	Info { version: String::from(version), title: String::from(title), ..Default::default() }
}
