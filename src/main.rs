use leptos::*;

#[derive(Debug, Clone)]
enum Color {
    White,
    Red
}

impl Color {
    pub fn reverse(color: &Color) -> Color {
        match color {
            Color::Red => Color::White,
            Color::White => Color::Red
        }
    }

    pub fn to_reversed(&mut self) {
        *self = Color::reverse(self) 
    }
}

impl IntoView for Color {
    fn into_view(self, cx: Scope) -> View {
        let color_str = format!("{:?}", self);
        color_str.into_view(cx)
    }
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let color = create_rw_signal(cx, Color::White);
    let next_color = move || Color::reverse(&color());

    view! { cx,
        <button
            on:click=move |_| {
                color.update(|c| c.to_reversed() );
            }
            class:red=move || matches!(color(), Color::Red)
        >
            "Click me: "
            { color }
        </button>
        <p>
            "Next color: " 
            { next_color }
        </p>
    }
}

fn main() {
    leptos::mount_to_body(|cx| view! { cx, <App/> })
}