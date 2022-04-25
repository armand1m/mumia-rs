use crate::api::{fetch_sounds::fetch_sounds, sound::Sound};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

pub fn use_sounds() -> UseStateHandle<Vec<Sound>> {
    let sounds = use_state(Vec::new);

    {
        let sounds = sounds.clone();
        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    let fetched_sounds = fetch_sounds().await.unwrap();
                    sounds.set(fetched_sounds);
                });
                || ()
            },
            (),
        );
    }

    sounds
}
