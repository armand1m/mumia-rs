use crate::api::{play_sound::play_sound, sound::Sound};
use crate::components::sounds_list::SoundsList;
use crate::hooks::lock::use_lock;
use crate::hooks::sounds::use_sounds;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let sounds = use_sounds();
    let is_locked = use_lock();
    let on_sound_click = {
        Callback::from(move |sound: Sound| {
            log::debug!("{:?}", sound);
            spawn_local(async move {
                if let Err(error) = play_sound(sound).await {
                    log::error!("Failed to play sound {:?}", error);
                }
            });
        })
    };

    let c_sounds = (*sounds).clone();

    html! {
        <SoundsList sounds={c_sounds} on_click={on_sound_click} is_locked={*is_locked} />
    }
}
