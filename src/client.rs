mod pb;

use futures::{SinkExt, StreamExt};
use pb::*;
use tokio::net::TcpStream;
use tokio_util::codec::LengthDelimitedCodec;
use tracing::info;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    info!("test tracing");

    let addr = "127.0.0.1:8888";

    let stream = TcpStream::connect(addr).await?;

    let mut stream = LengthDelimitedCodec::builder()
        .length_field_length(2)
        .new_framed(stream);

    let msg = Request::new_put("test_key", b"world");
    stream.send(msg.into()).await?;

    let msg = Request::new_get("test_key");
    stream.send(msg.into()).await?;

    while let Some(Ok(buf)) = stream.next().await {
        // 换一种写法，用 tryfrom 把 buf convert 成 msg
        let msg = Response::try_from(buf)?; // 等同于 buf.try_into()
        println!("Got msg: {:?}", msg);
    } 

    Ok(())
}
