use stylist::yew::styled_component;
use yew::prelude::*;

use crate::contexts::theme::{use_theme, ThemeKind};

#[styled_component(SwitchThemeButton)]
pub fn switch_theme_button() -> Html {
    let theme = use_theme();

    let theme_str = match theme.kind() {
        ThemeKind::Light => "Dark Theme",
        ThemeKind::Dark => "Light Theme",
    };

    let other_theme = match theme.kind() {
        ThemeKind::Light => ThemeKind::Dark,
        ThemeKind::Dark => ThemeKind::Light,
    };

    let switch_theme = Callback::from(move |_| theme.set(other_theme.clone()));

    let classes = css!(
        r#"color: white;
        height: 50px;
        width: 300px;
        font-size: 20px;
        background-color: rgb(88, 164, 255);
        border-radius: 5px;
        border: none;
    "#
    );

    html! {
        <button class={classes} onclick={switch_theme}>
            {"Switch to "}{theme_str}
        </button>
    }
}
