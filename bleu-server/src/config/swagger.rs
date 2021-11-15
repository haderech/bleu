use paperclip::v2::models::{DefaultApiRaw, Info, Tag};

pub struct SwaggerConfig {
    spec: DefaultApiRaw,
}

impl SwaggerConfig {
    pub fn load() -> Self {
        let mut spec = DefaultApiRaw::default();
        spec.tags = SwaggerSpec::simple_tags(vec!["TxBatch", "StateRootBatch", "Tx", "L1ToL2", "TxLogs", "BoardSummary"]);
        spec.info = SwaggerSpec::simple_info("0.1", "Bleu Server");

        Self {
            spec,
        }
    }

    pub fn get_spec(&self) -> DefaultApiRaw {
        self.spec.to_owned()
    }
}

struct SwaggerSpec;

impl SwaggerSpec {
    fn simple_tags(tag_names: Vec<&str>) -> Vec<Tag> {
        let mut tags = Vec::new();
        for tag_name in tag_names.into_iter() {
            let tag = Tag {
                name: tag_name.to_string(),
                description: None,
                external_docs: None,
            };
            tags.push(tag);
        }
        tags
    }

    fn simple_info(version: &str, title: &str) -> Info {
        Info {
            version: String::from(version),
            title: String::from(title),
            ..Default::default()
        }
    }
}