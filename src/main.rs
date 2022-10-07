use std::time::Duration;

use actix_web::{guard, http::header::HeaderMap, web, web::Data, App, Result};
use actix_web::{HttpRequest, HttpResponse, HttpServer};
use async_graphql::http::GraphiQLSource;
use async_graphql::*;
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse, GraphQLSubscription};
use tokio_stream::*;
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

struct Subscription;

#[Subscription]
impl Subscription {
    async fn integers(&self, #[graphql(default = 1)] step: i32) -> impl Stream<Item = i32> {
        println!("received");
        let mut value = 0;
        tokio_stream::wrappers::IntervalStream::new(tokio::time::interval(Duration::from_secs(1)))
            .map(move |_| {
                println!("did a thing");
                value += step;
                value
            })
    }
}

struct Query;
#[Object]
impl Query {
    async fn add(&self, a: i32, b: i32) -> Special {
        Special { a, b }
    }
}
type TestSchema = Schema<Query, EmptyMutation, Subscription>;
async fn handler(schema: web::Data<TestSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn handler_graphiql() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            GraphiQLSource::build()
                .endpoint("http://localhost:8080")
                .subscription_endpoint("ws://localhost:8080")
                .finish(),
        ))
}

async fn handler_ws(schema:web::Data<TestSchema>, req:HttpRequest, payload:web::Payload) -> Result<HttpResponse>{
    GraphQLSubscription::new(Schema::clone(&*schema)).start(&req, payload)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let schema = Schema::new(Query, EmptyMutation, Subscription);

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema.clone()))
            .service(web::resource("/").guard(guard::Post()).to(handler))
            .service(web::resource("/").guard(guard::Get()).guard(guard::Header("upgrade", "websocket")).to(handler_ws))
            .service(web::resource("/").guard(guard::Get()).to(handler_graphiql))
    })
    .bind("localhost:8080")?
    .run()
    .await
}

