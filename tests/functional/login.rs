use actix_web::{http::{HeaderValue, HeaderName}, test};
use awesome_rust_web_app::app;
use serde_json::json;

#[actix_rt::test]
async fn test_login() {
    app::setup(app::AppEnv::Test);

    let mut service = test::init_service(app::create()).await;

    let message = json!({
        "username": "sam.smith",
        "password": "1234Qwer",
    });

    let req = test::TestRequest::post()
        .uri("/api/login")
        .insert_header((HeaderName::from_static("accept-language"), HeaderValue::from_static( "en-US,en;q=0.9,zh-CN;q=0.8,zh;q=0.7")))
        .set_json(&message)
        .to_request();

    let resp = test::call_service(&mut service, req).await;

    assert!(resp.status().is_success());
}
