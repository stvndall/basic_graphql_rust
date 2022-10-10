use async_graphql::{ComplexObject, Context, Enum, Object, SimpleObject, ID};

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Product {
    id: ID,
}

#[ComplexObject]
impl Product {
    async fn review(&self) -> Vec<String> {
        vec!["sadfasdfa".to_string(), "lkdmlkafd".to_string()]
    }
}

pub struct RootQuery;
#[Object]
impl RootQuery {
    async fn um(&self) -> usize {
        1
    }
    #[graphql(entity)]
    async fn find_product_by_id(&self, #[graphql(key)] id: ID) -> Product {
        Product { id }
    }
}
