use fluent_langneg::negotiate_languages as fluent_negotiate_languages;
use fluent_langneg::parse_accepted_languages;
use fluent_langneg::NegotiationStrategy;
use fluent_templates::once_cell::sync::Lazy;
use fluent_templates::{Loader, StaticLoader};
use unic_langid::LanguageIdentifier;
use std::fmt;
use std::ops::Deref;

pub struct Negotiator<'a> {
    pub locale_loader: &'a Lazy<StaticLoader>,
    pub default_locale: &'a LanguageIdentifier,
}

impl<'a> Negotiator<'a> {
    // Find the matched languages, The weigh property ignored for the fluent_langneg removed it.
    // Send a request to fluent_langneg if you need this feature.
    pub fn negotiate(&self, accept_language: &str) -> MatchedLocales {
        let requested = parse_accepted_languages(accept_language);

        let available_locales: Vec<LanguageIdentifier> =
            self.locale_loader.locales().map(|l| l.to_owned()).collect();
        
        let matched =  fluent_negotiate_languages(
            &requested,
            &available_locales,
            Some(self.default_locale),
            NegotiationStrategy::Filtering,
        );

        MatchedLocales(matched.iter().map(|l|l.as_ref().to_owned()).collect())
    }
}


pub struct MatchedLocales(Vec<LanguageIdentifier>);

impl fmt::Display for MatchedLocales {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let stringified= self.0
        .iter()
        .map(|l|l.to_string())
        .collect::<Vec<String>>()
        .join(";");

        write!(f, "{}", stringified)
    }
}

impl Deref for MatchedLocales {
    type Target = Vec<LanguageIdentifier>;
    fn deref(&self) -> &Self::Target{
        &self.0
    }
}