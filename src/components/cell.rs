use leptos::*;

#[component]
pub fn Cell(cx: Scope, r_color: ReadSignal<&'static str>) -> impl IntoView {
    view! { cx,
        <div
            style:width = move || 20
            style:height = move || 20
            style:background-color={r_color}
            style:border = "1px solid #595959"
        >
            <br/>
        </div>
    }
}
