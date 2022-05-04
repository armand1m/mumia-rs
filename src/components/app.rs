use crate::api::{play_sound::play_sound, sound::Sound};
use crate::components::sounds_list::SoundsList;
use crate::components::switch_theme_button::SwitchThemeButton;
use crate::contexts::theme::{use_theme, ThemeKind};
use crate::hooks::lock::use_lock;
use crate::hooks::sounds::use_sounds;
use stylist::yew::{styled_component, Global};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[styled_component(App)]
pub fn app() -> Html {
    let sounds = use_sounds();
    let is_locked = use_lock();
    let on_sound_click = Callback::from(move |sound: Sound| {
        log::debug!("{:?}", sound);
        spawn_local(async move {
            if let Err(error) = play_sound(sound).await {
                log::error!("Failed to play sound {:?}", error);
            }
        });
    });

    let c_sounds = (*sounds).clone();

    let theme = use_theme();

    let theme_str = match theme.kind() {
        ThemeKind::Light => "light theme",
        ThemeKind::Dark => "dark theme",
    };
    html! {
        <>
             <Global css={css!(
                r#"
                    html, body {
                        font-family: sans-serif;
                        padding: 0;
                        margin: 0;
                        display: flex;
                        justify-content: center;
                        align-items: center;
                        min-height: 100vh;
                        flex-direction: column;
                        background-color: ${bg};
                        color: ${ft_color};
                    }
                "#,
                bg = theme.background_color.clone(),
                ft_color = theme.font_color.clone(),
            )} />
            <div>
                {"You are now using the "}{theme_str}{"!"}
            </div>
            <SwitchThemeButton />
            <SoundsList
                sounds={c_sounds}
                on_click={on_sound_click}
                is_locked={*is_locked}
            />
        </>
    }
}
