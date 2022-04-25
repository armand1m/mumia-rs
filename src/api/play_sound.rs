use crate::api::sound::Sound;
use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;

#[derive(Deserialize, Serialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
struct PlaySoundBody {
    client: String,
    sound_id: String,
}

impl From<PlaySoundBody> for JsValue {
    fn from(body: PlaySoundBody) -> Self {
        serde_json::to_string(&body).unwrap().into()
    }
}

pub async fn play_sound(sound: Sound) -> Result<(), reqwasm::Error> {
    let resp = Request::post("https://muminst-server.d1m.dev/play-sound")
        .body(PlaySoundBody {
            client: "discord".to_string(),
            sound_id: sound.id,
        })
        .header("Content-Type", "application/json")
        .send()
        .await
        .unwrap();

    let _ = resp.text().await?;

    Ok(())
}
