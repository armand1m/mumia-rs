use crate::api::sound::Sound;
use crate::components::sound_button::SoundButton;
use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct SoundsListProps {
    pub sounds: Vec<Sound>,
    pub on_click: Callback<Sound>,
    pub is_locked: bool,
}

#[styled_component(SoundsList)]
pub fn sounds_list(
    SoundsListProps {
        sounds,
        on_click,
        is_locked,
    }: &SoundsListProps,
) -> Html {
    let on_click = on_click.clone();
    let disabled = *is_locked;

    sounds
        .iter()
        .map(|sound| {
            html! {
                <SoundButton
                    sound={sound.clone()}
                    on_click={on_click.clone()}
                    disabled={disabled}
                />
            }
        })
        .collect()
}
