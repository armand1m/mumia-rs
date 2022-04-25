use crate::api::sound::Sound;
use reqwasm::http::Request;

pub async fn fetch_sounds() -> Result<Vec<Sound>, reqwasm::Error> {
    let resp = Request::get("https://muminst-server.d1m.dev/sounds")
        .send()
        .await
        .unwrap();

    let payload = resp.json::<Vec<Sound>>().await?;

    Ok(payload)
}
