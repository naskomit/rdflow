// #![feature(adt_const_params)]
// #![feature(generic_const_exprs)]

use system::{SystemStorage, System};

mod system;
mod block;
mod block_library;
mod examples;

fn main() {
    let mut system = examples::temperature_controller::SystemImpl::new();
    println!("{}", system.storage.r_param_get(0));
    system.storage.r_param_set(0, 3.5);
    println!("{}", system.storage.r_param_get(0));
    println!("{}", system.storage.r_param_get(1));

    let mut i = 0;
    while i < 100 {
        system.step();
        i = i + 1;
    }

}

