use actix_web::http::header::{AcceptLanguage, LanguageTag, qitem, QualityItem};
use fluent_langneg::negotiate_languages as fluent_negotiate_languages;
use fluent_langneg::parse_accepted_languages;
use fluent_langneg::NegotiationStrategy;
use fluent_templates::once_cell::sync::Lazy;
use fluent_templates::{Loader, StaticLoader};
use unic_langid::LanguageIdentifier;

pub struct Negotiator<'a> {
    pub locale_loader: &'a Lazy<StaticLoader>,
    pub default_locale: &'a LanguageIdentifier,
}

impl<'a> Negotiator<'a> {
    pub fn negotiate(&self, accept_language: &str) -> String
    {
        self.negotiate_languages(accept_language)
        .iter()
        .map(|l|l.to_string())
        .collect::<Vec<String>>()
        .join(";")
    }

    fn negotiate_languages(&self, accept_language: &str) -> Vec<LanguageIdentifier> {
        let requested = parse_accepted_languages(accept_language);

        let available_locales: Vec<LanguageIdentifier> =
            self.locale_loader.locales().map(|l| l.to_owned()).collect();
        
        let matched =  fluent_negotiate_languages(
            &requested,
            &available_locales,
            Some(self.default_locale),
            NegotiationStrategy::Filtering,
        );

        matched.iter().map(|l|l.as_ref().to_owned()).collect()
    }
}

