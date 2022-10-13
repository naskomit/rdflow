use dataflow_core::block::{Parameter, Input, ContinuousState, Output};

struct ThermalMass<'a> {
  pub cp: Parameter<'a, f64>,
  pub h: Parameter<'a, f64>,
  pub area: Parameter<'a, f64>,
  pub t_amb: Parameter<'a, f64>,
  pub qdot: Input<'a, f64>,
  pub t: ContinuousState<'a, f64>,
  pub t_out: Output<'a, f64>,
}
impl<'a> ThermalMass<'a> {
  pub fn new(
      storage: &'a dyn dataflow_core::system::SystemStorage,
      counters: &mut dataflow_core::system::SystemCounters,
  ) -> ThermalMass<'a> {
      ThermalMass {
          cp: "4",
          h: "4",
          area: "4",
          t_amb: "4",
      }
  }
}
pub const SIZE: dataflow_core::block::BlockSize = dataflow_core::block::BlockSize {
  r_param: 4usize,
  b_param: 0usize,
  r_state: 1usize,
  b_state: 0usize,
  r_in: 1usize,
  b_in: 0usize,
  r_out: 1usize,
  b_out: 0usize,
};


fn main() {

}