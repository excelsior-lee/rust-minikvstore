mod pb;

use std::sync::Arc;

use futures::{SinkExt, StreamExt};
use pb::{request::*, *};
use prost::Message;

use anyhow::Result;
use dashmap::DashMap;
use tokio::net::{TcpListener, TcpStream};
use tokio_util::codec::LengthDelimitedCodec;
use tracing::info;

#[derive(Debug)]
struct ServerState {
    store: DashMap<String, Vec<u8>>,
}

impl ServerState {
    pub fn new() -> Self {
        Self {
            store: DashMap::new(),
        }
    }
}

impl Default for ServerState {
    fn default() -> Self {
        Self::new()
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    info!("test tracing");

    let state = Arc::new(ServerState::new());
    let addr = "0.0.0.0:8888";
    let listener = TcpListener::bind(addr).await?;

    info!("Listening on {:?}", addr);

    loop {
        let (stream, _) = listener.accept().await?;
        info!("New connection: {:?}", addr);

        let shared = state.clone();

        tokio::spawn(async move {
            let mut stream = LengthDelimitedCodec::builder()
                .length_field_length(2)// 头两个字节是 frame 里实际数据的长度
                .new_framed(stream);

            while let Some(Ok(buf)) = stream.next().await {
                let msg: Request = buf.try_into()?;
                info!("Got a command: {:?}", msg);

                let response = match msg.command {
                    Some(Command::Get(RequestGet { key })) => {
                        // 拿到请求的 key，从 state 中取出来并返回
                        match shared.store.get(&key) {
                            Some(v) => Response::new(key, v.value().to_vec()),
                            None => Response::not_found(key),
                        }
                    }
                    Some(Command::Put(RequestPut { key, value })) => {
                        shared.store.insert(key.clone(), value.clone());
                        Response::new(key, value)
                    }
                    None => unimplemented!(),
                };
                stream.send(response.into()).await?;
            }
            Ok::<(), anyhow::Error>(())
        });
    }

    Ok(())
}
