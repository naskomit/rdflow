use dataflow_core::block::{Parameter, Input, ContinuousState, Output};
use dataflow_macros::Block;

// #[derive(Block)]
// enum X1 {
//   A, B
// }

// #[derive(Block)]
// struct X2(i32, i32);

// struct P1<'a, T> {
//   t: &'a T
// }

#[allow(dead_code)]
#[derive(Block)]
struct ThermalMass<'a> {
  #[default = 100.0]
  pub cp: Parameter<'a, f64>,
  #[default = 10.0]
  pub h: Parameter<'a, f64>,
  #[default = 1.0]
  pub area: Parameter<'a, f64>,
  #[default = 1.0]
  pub t_amb: Parameter<'a, f64>,  
  pub qdot: Input<'a, f64>,
  #[initial = 20.0]
  pub t: ContinuousState<'a, f64>,
  pub t_out: Output<'a, f64>,
}

#[test]
fn tests() {
  //let mass = ThermalMass::new();
}