use async_graphql::Object;

struct Special {
    a: i32,
    b: i32,
}

#[Object]
impl Special {
    async fn this(&self) -> String {
        println!("received");
        "found me".to_string()
    }
    async fn that(&self) -> i32 {
        println!("received");
        self.a + self.b
    }
}

pub struct RootQuery;
#[Object]
impl RootQuery {
    async fn add(&self, a: i32, b: i32) -> Special {
        Special { a, b }
    }

    async fn something(&self) -> Option<Special> {
        None
    }
}
