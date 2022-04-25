use crate::api::sound::Sound;
use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct SoundsListProps {
    pub sounds: Vec<Sound>,
    pub on_click: Callback<Sound>,
}

#[function_component(SoundsList)]
pub fn sounds_list(SoundsListProps { sounds, on_click }: &SoundsListProps) -> Html {
    let on_click = on_click.clone();

    sounds.iter().map(|sound| {
        let on_sound_select = {
            let on_click = on_click.clone();
            let sound = sound.clone();
            Callback::from(move |_| {
                on_click.emit(sound.clone())
            })
        };

        html! {
            <button onclick={on_sound_select}>{format!("{}: {}", sound.name, sound.tags.join(","))}</button>
        }
    }).collect()
}
