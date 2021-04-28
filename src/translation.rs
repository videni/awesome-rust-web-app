use unic_langid::{LanguageIdentifier, langid};
use fluent_templates::{Loader, static_loader};

const US_ENGLISH: LanguageIdentifier = langid!("en-US");
const SIMPLE_CHINESE: LanguageIdentifier = langid!("zh-CN");

static_loader! {
    static LOCALES = {
        locales: "./translations",
        fallback_language: "en-US",
        // Optional: A fluent resource that is shared with every locale.
        core_locales: "./translations/core.ftl",
    };
}

pub fn trans(locale: &LanguageIdentifier, message: &str) -> String {
    LOCALES.lookup(locale, message)
}