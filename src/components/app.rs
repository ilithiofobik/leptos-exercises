use crate::components::footer::*;
use crate::grid::*;
use crate::state::wildfire::*;
use crate::state::IState;
use leptos::*;
use std::time::Duration;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    type AutomatonState = WildFireState;
    let (r_grid, w_grid) = create_signal(cx, Grid::<AutomatonState>::random_grid());

    let duration = Duration::from_millis(100);
    set_interval(
        move || {
            w_grid.update(|grid| grid.game_move());
        },
        duration,
    );

    let cell_divs = (0..ROWS)
        .map(|row| {
            (0..COLS)
                .map(|col| {
                    view! { cx,
                    <div
                        style:width = move || 20
                        style:height = move || 20
                        style:background-color=move || r_grid().get(row, col).to_color()
                        style:border = "1px solid #595959"
                    >
                    <br/>
                    </div>
                    }
                })
                .collect_view(cx)
        })
        .collect_view(cx);

    view! { cx,
        <h1>
            { AutomatonState::automaton_name() }
        </h1>

        <div
            class:grid=true
        >
            { cell_divs }
        </div>

        <Footer/>
    }
}
