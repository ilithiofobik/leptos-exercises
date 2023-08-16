use leptos::*;

#[derive(Clone, Copy)]
pub enum Mass {
    Real(f64),
    Infinite,
}

#[derive(Clone, Copy)]
pub struct Block {
    pub mass: Mass,
    pub x: f64,
    pub vx: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Block {
    pub fn new(mass: Mass, x: f64, vx: f64, y: f64, width: f64, height: f64) -> Self {
        Block {
            mass,
            x,
            vx,
            y,
            width,
            height,
        }
    }

    pub fn mass(&self) -> Mass {
        self.mass
    }

    pub fn vx(&self) -> f64 {
        self.vx
    }

    pub fn left(&self) -> f64 {
        self.x
    }

    pub fn right(&self) -> f64 {
        self.x + self.width
    }

    pub fn next_state<'a, I>(my_state: &'a Block, neighs: I, w_x: WriteSignal<f64>) -> (f64, f64)
    where
        I: Iterator<Item = &'a Block>,
    {
        let x1 = my_state.x;
        let v1 = my_state.vx;

        match my_state.mass {
            Mass::Infinite => {
                w_x.update(|x| *x = x1 + v1);
                (x1 + v1, v1)
            }
            Mass::Real(m1) => {
                let neigh = if v1 < 0.0 {
                    neighs
                        .cloned()
                        .find(|neigh| my_state.left() <= neigh.right() + 1.0 && neigh.right() <= my_state.right() + 1.0 )
                } else if v1 == 0.0 {
                    neighs
                        .cloned()
                        .find(|neigh| (my_state.left() <= neigh.right() + 1.0  && neigh.right() <= my_state.right() + 1.0 ) || (neigh.left() <= my_state.right()  + 1.0 && my_state.right() <= neigh.right() + 1.0 ))
                } else {
                    neighs
                        .cloned()
                        .find(|neigh| neigh.left() <= my_state.right() + 1.0  && my_state.right() <= neigh.right() + 1.0 )
                };

                match neigh {
                    Some(neigh) => match neigh.mass() {
                        Mass::Infinite => (x1, -v1),
                        Mass::Real(m2) => {
                            log!("I do have a neighbour");
                            let v2 = neigh.vx();
                            let u1 = (v1 * (m1 - m2) + 2.0 * m2 * v2) / (m1 + m2);
                            log!("My new v is {}", u1);
                            (x1, u1)
                        }
                    },
                    None => {
                        //log!("I have no neighbours and I am just going: x={}, v1={}", x1, v1);
                        w_x.update(|x| *x = x1 + v1);
                        (x1 + v1, v1)
                    }
                }
            }
        }
    }
}
