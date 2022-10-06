// #![feature(adt_const_params)]
// #![feature(generic_const_exprs)]

use system::{SystemStorage, ISystem};

mod system;
mod block;
mod block_library;
mod examples;

fn main() {
    let system = examples::temperature_controller::SystemImpl::new();

    let mut i = 0;
    while i < 100 {
        println!("{} {}", system.storage.r_state_get(0), system.storage.b_state_get(0));
        system.step();
        system.advance_continuous_state(0.1);
        i = i + 1;
    }

}

