use fluent_templates::{static_loader, Loader};
use unic_langid::{langid, LanguageIdentifier};

pub static FALLBACK_LANGUAGE: LanguageIdentifier = langid!("en-US");

static_loader! {
    pub static LOCALES = {
        locales: "./translations",
        fallback_language: "en-US",
        // Optional: A fluent resource that is shared with every locale.
        core_locales: "./translations/core.ftl",
    };
}

pub fn trans(locale: &LanguageIdentifier, message: &str) -> String {
    LOCALES.lookup(locale, message)
}
