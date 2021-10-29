use std::env;

use paperclip::v2::models::{DefaultApiRaw, Info, Tag};

pub struct SwaggerConfig {
    resource: String,
    port: String,
    spec: DefaultApiRaw,
}

impl SwaggerConfig {
    pub fn load() -> Self {
        let mut spec = DefaultApiRaw::default();
        spec.tags = SwaggerSpec::simple_tags(vec!["TxBatch", "StateRootBatch", "Tx", "L1ToL2", "TxLogs"]);
        spec.info = SwaggerSpec::simple_info("0.1", "Bleu Server");

        let resource = env::var("SWAGGER_RESOURCE").expect("SWAGGER_RESOURCE does not exist!");
        let port = env::var("SWAGGER_PORT").expect("SWAGGER_PORT does not exist!");

        Self {
            resource,
            port,
            spec,
        }
    }

    pub fn get_resource(&self) -> &str {
        self.resource.as_str()
    }

    pub fn get_port(&self) -> &str {
        self.port.as_str()
    }

    pub fn get_default_origin(&self) -> String {
        format!("http://localhost:{}", self.get_port())
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