use crate::components::collision::block::Block;
use crate::logic::collision::block::*;
use crate::logic::collision::board::*;
use leptos::*;
use std::time::Duration;

const DURATION: Duration = Duration::from_millis(25); // 40fps

#[component]
pub fn Stage(cx: Scope) -> impl IntoView {
    let (r_board, w_board) = create_signal(cx, Board::pi_calculator(cx));

    set_interval(
        move || {
            w_board.update(|board| board.game_move());
        },
        DURATION,
    );

    let len = r_board().len;
    let size = || window().inner_width().unwrap().as_f64().unwrap() as usize;
    let (r_size, w_size) = create_signal(cx, size());
    window_event_listener(ev::resize, move |_: ev::UiEvent| w_size(size()));

    let blocks = (0..len)
        .map(|idx| {
            let block = r_board().inner_a[idx];
            let rx = r_board().r_xs[idx];
            let width = block.width;
            let height = block.height;
            let y = block.y;
            view! { cx,
                <Block y=y width=width height=height rx=rx/>
            }
        })
        .collect_view(cx);

    view! { cx,
        <svg
            width=move || r_size()
            height=move || r_size() / 2
        >
            { blocks }
        </svg>
    }
}
