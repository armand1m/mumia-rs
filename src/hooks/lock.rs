use futures::StreamExt;
use reqwasm::websocket::futures::WebSocket;
use reqwasm::websocket::Message;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct LockMessage {
    is_locked: bool,
}

pub fn use_lock() -> UseStateHandle<bool> {
    let is_locked = use_state(|| false);

    {
        let is_locked = is_locked.clone();
        use_effect_with_deps(
            move |_| {
                let ws = WebSocket::open("wss://muminst-server.d1m.dev/ws").unwrap();
                let (mut _write, mut read) = ws.split();

                spawn_local(async move {
                    while let Some(msg) = read.next().await {
                        match msg {
                            Ok(Message::Text(payload)) => {
                                let lock_msg: LockMessage =
                                    serde_json::from_str(payload.as_str()).unwrap();
                                is_locked.set(lock_msg.is_locked);
                            }
                            Ok(_) => {}
                            Err(err) => {
                                log::error!("Websocket msg error: {:?}", err);
                            }
                        }
                    }
                    log::debug!("WebSocket Closed")
                });
                || ()
            },
            (),
        );
    }

    is_locked
}
