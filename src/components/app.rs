use crate::api::{fetch_sounds::fetch_sounds, play_sound::play_sound, sound::Sound};
use crate::components::sounds_list::SoundsList;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let sounds = use_state::<Vec<Sound>, _>(Vec::new);

    {
        // This block exists to clone and move the `sounds`
        // reference without having to rename it to something else
        let sounds = sounds.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_sounds = fetch_sounds().await.unwrap();
                    sounds.set(fetched_sounds);
                });
                || ()
            },
            (),
        );
    }

    let on_sound_click = {
        Callback::from(move |sound: Sound| {
            log::debug!("{:?}", sound);
            wasm_bindgen_futures::spawn_local(async move {
                if let Err(error) = play_sound(sound).await {
                    log::error!("Failed to play sound {:?}", error);
                }
            });
        })
    };

    let c_sounds = (*sounds).clone();

    html! {
        <SoundsList sounds={c_sounds} on_click={on_sound_click} />
    }
}
