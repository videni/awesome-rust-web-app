use async_trait::async_trait;

pub mod create_user_handler;
pub mod login_handler;

// Message handle for query and command
// generally, query always returns something, however command doesn't,
// but I don't handle it strictly as CQRS suggests for performance reason
#[async_trait]
pub trait MessageHandler {
    // The message to handle
    type Message;

    // The reply of your handler
    type Reply;

    //The error you handler might throw
    type Error;

    async fn handle(&self, msg: Self::Message) -> Result<Self::Reply, Self::Error>;
}
