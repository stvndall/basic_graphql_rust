use async_graphql::{EmptyMutation, Object};

pub struct RootMutation;

#[Object]
impl RootMutation{
    async fn is_empty(&self) -> Option<i32>{
        None
    }
}
