use std::{iter::Product, time::Duration};

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

    async fn prods(&self) -> impl Stream<Item = u32> {
        let mut c = 0;
        tokio_stream::wrappers::IntervalStream::new(tokio::time::interval(Duration::from_secs(2)))
            .map(move |_| {
                c += 1;
                c
            })
    }

    // async fn products(&self, #[graphql(default = 1)] step: i32) -> impl Stream<Item = Product> {
    //     println!("received");
    //     let mut value = 0;
    //     tokio_stream::wrappers::IntervalStream::new(tokio::time::interval(Duration::from_secs(1)))
    //         .map(move |_| {
    //             println!("did a thing");
    //             value += step;
    //             value
    //         })
    // }
}
