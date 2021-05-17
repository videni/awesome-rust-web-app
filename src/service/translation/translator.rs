use std::{collections::HashMap, marker::PhantomData};
use fluent_templates::{Loader, fluent_bundle::FluentValue, static_loader};
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

pub struct Translator<'a, T>
where T: AsRef<str>
{
    locale: &'a  LanguageIdentifier,
    _phantom: PhantomData<T>
}

// Translate the message using the specified locale
impl<'a, T> Translator<'a, T>
where T: AsRef<str>
{
    pub fn new(locale: &'a LanguageIdentifier) -> Self {
        Self {
            locale,
            _phantom:  PhantomData
        }
    }

    pub fn trans(&self, message: &str, args: &HashMap<T, FluentValue>,) -> String {
        LOCALES.lookup_with_args(self.locale, message, args)
    }

    pub fn trans_simple(&self, message: &str) -> String {
        LOCALES.lookup(self.locale, message)
    }

    pub fn trans_full(&self, message: &str, args: Option<&HashMap<T, FluentValue>>) -> String {
        LOCALES.lookup_complete(self.locale, message, args)
    }
}

