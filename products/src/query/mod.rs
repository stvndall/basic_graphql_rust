use async_graphql::{Object, SimpleObject, ID};


#[derive(SimpleObject)]
pub(crate) struct Product {
    id: ID,
    name: String,
}

impl Product {
    pub fn new(id: &str, name: &str) -> Self {
        Product {
            id: id.into(),
            name: name.to_string(),
        }
    }
}
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
