use async_graphql::Object;

pub struct RootMutation;

#[Object]
impl RootMutation {
    async fn is_empty(&self) -> Option<i32> {
        None
    }
}
