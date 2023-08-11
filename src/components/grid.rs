use crate::components::cell::*;
use crate::grid::*;
use crate::state::{wildfire::*, GameState};
use leptos::*;
use std::time::Duration;

#[component]
pub fn Grid(cx: Scope) -> impl IntoView {
    type AutomatonState = WildFireState;

    let rand_grid = Grid::<AutomatonState>::random_grid(cx);
    let (r_grid, w_grid) = create_signal(cx, rand_grid);

    let duration = Duration::from_millis(50);
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
                        <Cell r_color=r_grid().r_colors[row][col]/>
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
    }
}
