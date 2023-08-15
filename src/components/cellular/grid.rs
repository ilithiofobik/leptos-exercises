use crate::components::cellular::cell::Cell;
use crate::grid::*;
use crate::logic::cellular::state::*;
use crate::logic::cellular::wildfire::*;
use leptos::*;
use std::time::Duration;

const DURATION: Duration = Duration::from_millis(25); // 40fps

#[component]
pub fn Grid(cx: Scope) -> impl IntoView {
    type AutomatonState = WildFireState;

    let rand_grid = Grid::<AutomatonState>::random_grid(cx);
    let (r_grid, w_grid) = create_signal(cx, rand_grid);

    set_interval(
        move || {
            w_grid.update(|grid| grid.game_move());
        },
        DURATION,
    );

    let size = || window().inner_width().unwrap().as_f64().unwrap() as usize / COLS;
    let (r_size, w_size) = create_signal(cx, size());
    window_event_listener(ev::resize, move |_: ev::UiEvent| w_size(size()));

    let cell_divs = (0..ROWS)
        .map(|row| {
            (0..COLS)
                .map(|col| {
                    view! { cx,
                        <Cell row=row col=col r_size=r_size r_color=r_grid().r_colors[row][col]/>
                    }
                })
                .collect_view(cx)
        })
        .collect_view(cx);

    view! { cx,
        <h1 class="p-6 text-4xl text-center font-bold">
            { AutomatonState::automaton_name() }
        </h1>

        <svg
            width=move || COLS * r_size()
            height=move || ROWS * r_size()
        >
            { cell_divs }
        </svg>
    }
}
