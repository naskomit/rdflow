use dataflow_core::block::{Parameter, Input, ContinuousState, Output};
use dataflow_macros as mac;
use dataflow::block_library::{converter_b2f, thermal_mass, hysteresis};

// #[derive(Block)]
// enum X1 {
//   A, B
// }

// #[derive(Block)]
// struct X2(i32, i32);

// struct P1<'a, T> {
//   t: &'a T
// }

// #[allow(dead_code)]
// #[derive(mac::Block)]
// struct ThermalMass<'a> {
//   #[default = 100.0]
//   pub cp: Parameter<'a, f64>,
//   #[default = 10.0]
//   pub h: Parameter<'a, f64>,
//   #[default = 1.0]
//   pub area: Parameter<'a, f64>,
//   #[default = 1.0]
//   pub t_amb: Parameter<'a, f64>,  
//   pub qdot: Input<'a, f64>,
//   #[initial = 20.0]
//   pub t: ContinuousState<'a, f64>,
//   pub t_out: Output<'a, f64>,
// }

// use dataflow_core::system::{SystemSize, SystemStorage, ISystem, SystemCounters, static_storage};
// use dataflow_core::block::{IBlock, Access, DerivativeAccess, BlockComputation, UpdateComputation};
// use crate::block_library::{hysteresis, thermal_mass, converter_b2f};

// static_storage!(STORAGE, 
//   thermal_mass::Block<'a>, 
//   hysteresis::Block<'a>, 
//   converter_b2f::Block<'a>
// );
// #[allow(dead_code)]
// #[derive(mac::System)]
// struct TemperatureController<'a> {
//   mass: thermal_mass::Block<'a>,
//   hyst_component: hysteresis::Block<'a>,
//   b2f: converter_b2f::Block<'a>
// }


#[test]
fn tests() {
  //let mass = ThermalMass::new();
}