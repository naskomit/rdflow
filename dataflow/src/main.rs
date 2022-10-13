use dataflow_core::system::{SystemStorage, ISystem};
use dataflow::examples;
// use dataflow::const_fns;

fn main() {


  let system = examples::temperature_controller::SystemImpl::new();
  let storage = &system.storage;
  let computations = examples::temperature_controller::SystemImpl::computations(&system);
  std::process::Command::new("clear").status().unwrap();
  println!("======================== Begin simulation ========================");
  println!("=== Params ===");
  storage.print_params();

  println!("=== Initial ===");
  println!("{}", std::format!("States: [{:+.3e}, {}]; Outputs [{}, {:+.3e}, {:+.3e}]",
      storage.r_state_get(0), storage.b_state_get(0),
      storage.b_out_get(0), storage.r_out_get(0), storage.r_out_get(1)
  ));

  let mut i = 0;
  let mut t = 0.0;
  let dt = 5.0;

  println!("=== Loop ===");
  while i < 100 {
      t += dt;
      println!("=== t = {} (step {})", t, i);
      system.step(&computations);
      system.advance_continuous_state(dt);
      system.storage().print_states_outputs();
      i = i + 1;
  }

  // const_fns::test_constants();
}
