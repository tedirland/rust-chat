use std::{
    collections::HashMap,
    sync::atomic::{AtomicUsize, Ordering},
};

use chrono::Utc;
use common::{ChatMessage, WebSocketMessage, WebSocketMessageType};
use rocket::{
    futures::{stream::SplitSink, SinkExt, StreamExt},
    tokio::sync::Mutex,
    State,
};
use rocket_ws::{stream::DuplexStream, Channel, Message, WebSocket};
use serde_json::json;

static USER_ID_COUNTER: AtomicUsize = AtomicUsize::new(1);

#[derive(Default)]
struct ChatRoom {
    // wrap the Hashmap in a Mutex type to provide the ability to lock
    connections: Mutex<HashMap<usize, SplitSink<DuplexStream, Message>>>,
}

impl ChatRoom {
    // method to add a connection
    pub async fn add_connection(&self, id: usize, sink: SplitSink<DuplexStream, Message>) {
        // whenever a user connects, add them into the connections list
        let mut conns = self.connections.lock().await;
        conns.insert(id, sink);
    }
    // remove a connection
    pub async fn remove_connection(&self, id: usize) {
        let mut conns = self.connections.lock().await;
        conns.remove(&id);
    }
    // send a message
    pub async fn broadcast_message(&self, message: Message, author_id: usize) {
        let chat_message = ChatMessage {
            message: message.to_string(),
            author: format!("User # {}", author_id),
            created_at: Utc::now().naive_utc(),
        };
        let websocket_message = WebSocketMessage {
            message_type: WebSocketMessageType::NewMessage,
            message: Some(chat_message),
            users: None,
        };
        // lock connections mutex
        let mut conns = self.connections.lock().await;

        // iterate through the connections
        for (_key, conn) in conns.iter_mut() {
            // send the message to each connection
            let _ = conn
                .send(Message::Text(json!(websocket_message).to_string()))
                .await;
        }
    }

    pub async fn broadcast_user_list(&self) {
        let mut conns = self.connections.lock().await;
        let mut users = vec![];
        for (id, _) in conns.iter_mut() {
            // send the message to each connection
            users.push(format!("User # {}", id));
        }
        let websocket_message = WebSocketMessage {
            message_type: WebSocketMessageType::UsersList,
            message: None,
            users: Some(users),
        };
        for (_key, conn) in conns.iter_mut() {
            // send the message to each connection
            let _ = conn
                .send(Message::Text(json!(websocket_message).to_string()))
                .await;
        }
    }
}

#[rocket::get("/")]
fn chat<'r>(ws: WebSocket, state: &'r State<ChatRoom>) -> Channel<'r> {
    ws.channel(move |stream| {
        Box::pin(async move {
            let user_id = USER_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
            let (ws_sink, mut ws_stream) = stream.split();
            state.add_connection(user_id, ws_sink).await;
            state.broadcast_user_list().await;

            // whenever another websocket sends a message
            while let Some(message) = ws_stream.next().await {
                state.broadcast_message(message?, user_id).await;
            }
            state.remove_connection(user_id).await;
            state.broadcast_user_list().await;

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
