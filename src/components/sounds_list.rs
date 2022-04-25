use crate::api::sound::Sound;
use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct SoundsListProps {
    pub sounds: Vec<Sound>,
    pub on_click: Callback<Sound>,
    pub is_locked: bool,
}

#[function_component(SoundsList)]
pub fn sounds_list(
    SoundsListProps {
        sounds,
        on_click,
        is_locked,
    }: &SoundsListProps,
) -> Html {
    let on_click = on_click.clone();
    let disabled = is_locked.to_owned();

    sounds
        .iter()
        .map(|sound| {
            let on_sound_select = {
                let on_click = on_click.clone();
                let sound = sound.clone();
                Callback::from(move |_| on_click.emit(sound.clone()))
            };

            html! {
                <button
                    onclick={on_sound_select}
                    disabled={disabled}
                >
                    {format!("{}: {}", sound.name, sound.tags.join(","))}
                </button>
            }
        })
        .collect()
}
