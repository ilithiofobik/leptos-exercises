use crate::logic::collision::block::*;
use leptos::*;

const GRAY: &str = "rgb(127,127,127)";

#[component]
pub fn Block(cx: Scope, y: f64, width: f64, height: f64, rx: ReadSignal<f64>) -> impl IntoView {
    view! { cx,
        <rect
            x=move || rx().round() as usize
            y=y
            width=width
            height=height
            style:fill=GRAY
        />
    }
}
