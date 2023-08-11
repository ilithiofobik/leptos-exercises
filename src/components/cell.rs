use leptos::*;

const GRAY: &str = "rgb(127,127,127)";

#[component]
pub fn Cell(
    cx: Scope,
    row: usize,
    col: usize,
    r_size: ReadSignal<usize>,
    r_color: ReadSignal<&'static str>,
) -> impl IntoView {
    view! { cx,
        <rect
            x=move || col * r_size()
            y=move || row * r_size()
            width=r_size
            height=r_size
            style:fill=r_color
            style:stroke-width=1
            style:stroke=GRAY
        />
    }
}
