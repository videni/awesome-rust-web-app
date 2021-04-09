use crate::db::ConnectionPool;
use actix::prelude::{Actor, SyncContext};

pub mod login_handler;

pub struct MessageHandler
{
    pub db_connection_pool: ConnectionPool,
}

impl Actor for MessageHandler {
    type Context = SyncContext<Self>;
}