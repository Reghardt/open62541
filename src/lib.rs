mod client;
mod data_type;
mod error;
pub mod ua;

pub(crate) use self::data_type::{data_type, DataType};
pub use self::{
    client::{Client, ClientBuilder},
    error::Error,
};

#[derive(Clone, Copy)]
pub struct SubscriptionId(u32);
