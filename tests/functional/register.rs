use actix_web::test;
use awesome_rust_web_app::app;
use serde_json::json;

#[actix_rt::test]
async fn test_register() {
    app::setup(app::AppEnv::Test);

    let mut service = test::init_service(app::create()).await;

    let message = json!({
        "username": "sam.smith",
        "email": "sam.smith@example.com",
        "password": "1234Qwer",
    });

    let req = test::TestRequest::post()
        .uri("/api/register")
        .set_json(&message)
        .to_request();

    let resp = test::call_service(&mut service, req).await;

    assert!(resp.status().is_success());
}
