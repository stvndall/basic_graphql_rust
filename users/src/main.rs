mod mutations;
mod query;
mod subscriptions;

use actix_web::{guard, web, web::Data, App, Result};
use actix_web::{HttpRequest, HttpResponse, HttpServer};
use async_graphql::http::GraphiQLSource;
use async_graphql::*;
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse, GraphQLSubscription};
use mutations::RootMutation;
use query::RootQuery;
use subscriptions::RootSubscription;

type ServiceSchema = Schema<RootQuery, RootMutation, RootSubscription>;

async fn handler(schema: web::Data<ServiceSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn handler_graphiql() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            GraphiQLSource::build()
                .endpoint("http://localhost:8083")
                .subscription_endpoint("ws://localhost:8083")
                .finish(),
        ))
}

async fn handler_ws(
    schema: web::Data<ServiceSchema>,
    req: HttpRequest,
    payload: web::Payload,
) -> Result<HttpResponse> {
    GraphQLSubscription::new(Schema::clone(&*schema)).start(&req, payload)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let schema = Schema::new(RootQuery, RootMutation, RootSubscription)
        .enable_federation()
        .finish();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema.clone()))
            .service(web::resource("/").guard(guard::Post()).to(handler))
            .service(
                web::resource("/")
                    .guard(guard::Get())
                    .guard(guard::Header("upgrade", "websocket"))
                    .to(handler_ws),
            )
            .service(web::resource("/").guard(guard::Get()).to(handler_graphiql))
    })
    .bind("localhost:8083")?
    .run()
    .await
}
