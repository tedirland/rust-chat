use std::{
    collections::HashMap,
    sync::atomic::{AtomicUsize, Ordering},
};

use rocket::{
    futures::{stream::SplitSink, SinkExt, StreamExt},
    tokio::sync::Mutex,
    State,
};
use rocket_ws::{stream::DuplexStream, Channel, Message, WebSocket};

static USER_ID_COUNTER: AtomicUsize = AtomicUsize::new(1);

#[derive(Default)]
struct ChatRoom {
    // wrap the Hashmap in a Mutex type to provide the ability to lock
    connections: Mutex<HashMap<usize, SplitSink<DuplexStream, Message>>>,
}

#[rocket::get("/")]
fn chat<'r>(ws: WebSocket, state: &'r State<ChatRoom>) -> Channel<'r> {
    ws.channel(move |stream| {
        Box::pin(async move {
            // this block represents when a user is connected
            let user_id = USER_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
            let (ws_sink, mut ws_stream) = stream.split();
            {
                // whenever a user connects, add them into the connections list
                let mut conns = state.connections.lock().await;
                conns.insert(user_id, ws_sink);
            }
            // whenever another websocket sends a message
            while let Some(message) = ws_stream.next().await {
                // block scope to allow mutex lock to expire
                {
                    // lock connections mutex
                    let mut conns = state.connections.lock().await;
                    let msg = message?;
                    // iterate through the connections
                    for (_key, conn) in conns.iter_mut() {
                        // send the message to each connection
                        let _ = conn.send(msg.clone()).await;
                    }
                }
            }
            {
                // remove each connection from the connections hashmap
                let mut conns = state.connections.lock().await;
                conns.remove(&user_id);
            }

            Ok(())
        })
    })
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", rocket::routes![chat])
        .manage(ChatRoom::default())
        .launch()
        .await;
}
