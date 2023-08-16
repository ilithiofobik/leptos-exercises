use crate::logic::collision::block::*;
use leptos::*;

#[derive(Clone)]
enum InnerVec {
    A,
    B,
}

impl InnerVec {
    fn flip(&mut self) {
        *self = match self {
            InnerVec::A => InnerVec::B,
            InnerVec::B => InnerVec::A,
        };
    }
}

#[derive(Clone)]
pub struct Board {
    pub inner_a: Vec<Block>,
    inner_b: Vec<Block>,
    pub len: usize,
    current: InnerVec,
    pub r_xs: Vec<ReadSignal<f64>>,
    w_xs: Vec<WriteSignal<f64>>,
}

impl Board {
    pub fn pi_calculator(cx: Scope) -> Board {
        let wall = Block::new(Mass::Infinite, 100.0, 0.0, 100.0, 100.0, 500.0);
        let small = Block::new(Mass::Real(1.0), 300.0, 0.0, 500.0, 100.0, 100.0);
        let big = Block::new(Mass::Real(100.0), 410.0, -1.0, 500.0, 100.0, 100.0);

        let inner_a = vec![wall, small, big];
        let inner_b = inner_a.clone();
        let current = InnerVec::A;

        let len = inner_a.len();
        let mut r_xs = Vec::with_capacity(len);
        let mut w_xs = Vec::with_capacity(len);

        for idx in 0..len {
            let (r_x, w_x) = create_signal(cx, (inner_a[idx]).x);
            r_xs.push(r_x);
            w_xs.push(w_x);
        }

        Board {
            inner_a,
            inner_b,
            current,
            len,
            r_xs,
            w_xs,
        }
    }

    fn get(&self, idx: usize) -> &Block {
        match self.current {
            InnerVec::A => &self.inner_a[idx],
            InnerVec::B => &self.inner_b[idx],
        }
    }

    fn set_prev(&mut self, idx: usize, x: f64, vx: f64) {
        match self.current {
            InnerVec::A => {
                (self.inner_b[idx]).x = x;
                (self.inner_b[idx]).vx = vx;
            }
            InnerVec::B => {
                (self.inner_a[idx]).x = x;
                (self.inner_a[idx]).vx = vx;
            }
        }
    }

    pub fn game_move(&mut self) {
        for idx in 0..self.len {
            let neighs_idxs = (0..idx).chain(idx + 1..self.len);
            let my_state = self.get(idx);
            let neighs = neighs_idxs.map(|i| self.get(i));
            let (x, vx) = Block::next_state(my_state, neighs, self.w_xs[idx]);

            self.set_prev(idx, x, vx);
        }
        self.current.flip();
    }
}
