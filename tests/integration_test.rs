use crate::app;
use crate::message::Register;
use actix_web::{test};

#[actix_rt::test]
async fn test_register() {
    let app = app::init();
    let mut service = test::init_service(
        app
        ).await;
    let req = test::TestRequest::post()
        .uri("/api/register")
        .set_json(Register {
            username: "sam.smith",
            email: "sam.smith@example.com",
            password: "1234Qwer",
        })
        .to_request();
    let resp = test::call_service(&mut service, req).await;
    assert!(resp.status().is_success());
}