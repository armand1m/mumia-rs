use crate::components::app::App;
use crate::contexts::theme::ThemeProvider;
use stylist::yew::styled_component;
use yew::prelude::*;

#[styled_component(Root)]
pub fn root() -> Html {
    html! {
        <ThemeProvider>
            <App />
        </ThemeProvider>
    }
}
