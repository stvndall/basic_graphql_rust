use async_graphql::{Object, SimpleObject, ID};

#[derive(SimpleObject)]
struct Product {
    id: ID,
    name: String,
}

impl Product {}

pub struct RootQuery;
#[Object]
impl RootQuery {
    async fn product(&self) -> Product {
        Product {
            id: "lksndfklsadflskjdfhkasjdfh".into(),
            name: "something something name".to_owned(),
        }
    }
}
