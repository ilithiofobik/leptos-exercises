use crate::components::cellular::grid::Grid;
use crate::components::footer::Footer;
use leptos::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <Grid/>
        <Footer/>
    }
}
