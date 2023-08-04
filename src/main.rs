use leptos::*;
use chrono::{Local, Datelike};

#[derive(Debug, Clone)]
enum Color {
    White,
    Red
}

impl Color {
    pub fn to_reversed(color: &Color) -> Color {
        match color {
            Color::Red => Color::White,
            Color::White => Color::Red
        }
    }

    pub fn reverse(&mut self) {
        *self = Color::to_reversed(self) 
    }
}

impl IntoView for Color {
    fn into_view(self, cx: Scope) -> View {
        let color_str = format!("{:?}", self);
        color_str.into_view(cx)
    }
}

#[component]
fn Footer(cx: Scope) -> impl IntoView{ 
    let year = Local::now().year();

    view! { cx,
        <footer>
            <p class="copyright">
                "©" { year } " Wojciech Sęk"
            </p>
        </footer>
    }
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let color = create_rw_signal(cx, Color::White);
    let next_color = move || Color::to_reversed(&color());

    view! { cx,
        <button
            on:click=move |_| {
                color.update(|c| c.reverse() );
            }
            class:red=move || matches!(color(), Color::Red)
        >
            "Click me: " { color }
        </button>
        
        <p>
            "Next color: " { next_color }
        </p>

        <Footer/>
    }
}

fn main() {
    leptos::mount_to_body(|cx| view! { cx, <App/> })
}