use crate::query::Product;
use async_graphql::Object;

pub struct RootMutation;

#[Object]
impl RootMutation {
    async fn is_empty(&self) -> Option<i32> {
        None
    }

    async fn udpate(&self) -> Product {
        Product::new("sadfasdf", "lsadflkasdf")
    }
}
