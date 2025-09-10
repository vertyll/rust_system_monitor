use fluent::FluentResource;
use fluent::concurrent::FluentBundle;
use std::fs;
use std::path::Path;

use crate::enums::supported_language_enum::SupportedLanguageEnum;

pub struct I18nManager {
    bundle: FluentBundle<FluentResource>,
}

impl I18nManager {
    pub fn new(lang: SupportedLanguageEnum) -> Self {
        let lang_id = lang.id();

        let ftl_path_str = format!("translations/{}/main.ftl", lang_id);
        let ftl_path = Path::new(&ftl_path_str);

        let ftl_string = fs::read_to_string(ftl_path).unwrap_or_else(|_| {
            println!(
                "Warning: Language file for '{}' not found. Falling back to 'en-US'.",
                lang
            );
            fs::read_to_string("translations/en-US/main.ftl")
                .expect("Failed to read fallback FTL file (en-US)")
        });

        let resource = FluentResource::try_new(ftl_string).expect("Failed to parse FTL string");

        let mut bundle = FluentBundle::new_concurrent(vec![lang_id]);
        bundle
            .add_resource(resource)
            .expect("Failed to add FTL resource to bundle");

        Self { bundle }
    }

    pub fn get_message(&self, id: &str) -> String {
        let msg = self
            .bundle
            .get_message(id)
            .expect(&format!("Message '{}' not found", id));
        let mut errors = vec![];
        let pattern = msg.value().expect("Message has no value");

        self.bundle
            .format_pattern(pattern, None, &mut errors)
            .to_string()
    }
}
