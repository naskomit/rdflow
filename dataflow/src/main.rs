
mod system;
mod component;
mod component_library;


fn main() {
    let mut system_size = system::SystemSize::new();
    system_size.r_param(2).b_state(1).b_out(1);
    println!("{:?}", system_size);
    let mut system = system::System::new(system_size);
    println!("{:?}", system);
}