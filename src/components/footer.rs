use chrono::{Datelike, Local};
use leptos::*;

#[component]
pub fn Footer(cx: Scope) -> impl IntoView {
    let year = Local::now().year();

    view! { cx,
        <footer>
            <p>
                "©" { year } " Wojciech Sęk"
            </p>
        </footer>
    }
}
