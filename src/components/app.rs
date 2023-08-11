use crate::components::footer::Footer;
use crate::components::grid::Grid;
use leptos::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <Grid/>
        <Footer/>
    }
}
