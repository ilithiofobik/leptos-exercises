use chrono::{Datelike, Local};
use leptos::*;

#[component]
pub fn Footer(cx: Scope) -> impl IntoView {
    let year = Local::now().year();

    view! { cx,
        <footer>
            <p class="text-sm text-violet-900">
                "©" { year } " Wojciech Sęk"
            </p>
        </footer>
    }
}
