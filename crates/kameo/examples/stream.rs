use std::{future::pending, time};

use futures::stream;
use kameo::{Actor, BoxError, Message};
use tokio_stream::StreamExt;
use tracing::info;
use tracing_subscriber::EnvFilter;

#[derive(Default)]
pub struct MyActor {
    count: i64,
}

impl Actor for MyActor {
    async fn on_start(&mut self) -> Result<(), BoxError> {
        let stream = Box::pin(stream::repeat(1)
            .take(5)
            .throttle(time::Duration::from_secs(1)));
        self.actor_ref().attach_stream(stream).await?;

        let stream = stream::repeat(1).take(5);
        self.actor_ref().attach_stream(stream).await?;
        Ok(())
    }
}

impl Message<i64> for MyActor {
    type Reply = ();

    async fn handle(&mut self, msg: i64) {
        self.count += msg as i64;
        info!("Count is {}", self.count);
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter("trace".parse::<EnvFilter>().unwrap())
        .without_time()
        .with_target(false)
        .init();

    kameo::spawn(MyActor::default());

    pending().await
}
