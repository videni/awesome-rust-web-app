use actix_web::{HttpMessage, dev::{Service, ServiceRequest, ServiceResponse, Transform}, http::HeaderName};
use actix_web::Error;
use actix_web::{
    http,
    http::header::HeaderValue,
};
use futures::future::{ok, Ready};
use futures::Future;
use std::{task::{Context, Poll}};
use std::{pin::Pin};
use super::negotiator::MatchedLocales;
pub use super::negotiator::Negotiator;
use unic_langid::LanguageIdentifier;
use crate::service::translation::translator;

static NEGOTIATOR: Negotiator = Negotiator {
    locale_loader: &translator::LOCALES,
    default_locale: &translator::FALLBACK_LANGUAGE,
};

pub struct NegotiateLanguage<'a>(&'a Negotiator<'a>);

impl<'a> Default for NegotiateLanguage<'a> {
    fn default() -> Self {
        NegotiateLanguage(&NEGOTIATOR)
    }
}

impl<'a, S, B> Transform<S, ServiceRequest> for NegotiateLanguage<'a>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = NegotiateLanguageMiddleware<'a, S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(NegotiateLanguageMiddleware {
            service,
            negotiator: &self.0,
        })
    }
}

// This middleware will try to set locale to request object if no locale present in the request
pub struct NegotiateLanguageMiddleware<'a, S> {
    service: S,
    negotiator: &'a Negotiator<'a>,
}

impl<'a, S, B> Service<ServiceRequest> for NegotiateLanguageMiddleware<'a, S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let mut negotiated_accept_language : Option<MatchedLocales> = None;
        if let Some(accept_language) = req.headers().get(http::header::ACCEPT_LANGUAGE) {
            if let Ok(accept_language) = accept_language.to_str() {
                let matched_locales = self.negotiator.negotiate(accept_language);

                let owned = matched_locales
                    .first()
                    .cloned();
                
                //Store the first locale into request
                if !req.extensions().contains::<Option<LanguageIdentifier>>(){
                    req.extensions_mut().insert(owned);
                }

                negotiated_accept_language = Some(matched_locales);
            }
        }

        let fut = self.service.call(req);

        Box::pin(async move {
            let mut res = fut.await?;

            if let Some(accepted_lanuage) = negotiated_accept_language {
                res.headers_mut()
                    .insert(HeaderName::from_static("accept-language"), HeaderValue::from_str(&accepted_lanuage.to_string()).unwrap());
                
                return Ok(res);
            }
           
            Ok(res)
        })
    }
}
