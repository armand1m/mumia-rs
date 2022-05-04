use crate::api::sound::Sound;
use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct SoundButtonProps {
    pub sound: Sound,
    pub on_click: Callback<Sound>,
    pub disabled: bool,
}

#[styled_component(SoundButton)]
pub fn sound_button(
    SoundButtonProps {
        sound,
        on_click,
        disabled,
    }: &SoundButtonProps,
) -> Html {
    let classes = css!(
        r#"
        color: white;
        height: 50px;
        width: 300px;
        font-size: 20px;
        background-color: rgb(88, 164, 255);
        border-radius: 5px;
        border: none;
    "#
    );

    let on_sound_click = {
        let on_click = on_click.clone();
        let sound = sound.clone();
        Callback::from(move |_| on_click.emit(sound.clone()))
    };

    html! {
        <button
            class={classes}
            onclick={on_sound_click}
            disabled={*disabled}
        >
            {format!("{}: {}", sound.name, sound.tags.join(","))}
        </button>
    }
}
