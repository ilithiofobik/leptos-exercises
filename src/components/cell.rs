use leptos::*;

const GRAY: &str = "rgb(127,127,127)";

#[component]
pub fn Cell(
    cx: Scope,
    row: usize,
    col: usize,
    size: usize,
    r_color: ReadSignal<&'static str>,
) -> impl IntoView {
    view! { cx,
        <rect
            x=col*size
            y=row*size
            width=size
            height=size
            style:fill=r_color
            style:stroke-width=1
            style:stroke=GRAY
        />
    }
}
