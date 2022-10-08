use std::time::Duration;

use async_graphql::Subscription;
use tokio_stream::{Stream, StreamExt};

pub struct RootSubscription;

#[Subscription]
impl RootSubscription {
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
