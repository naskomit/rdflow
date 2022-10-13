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
impl ThermalMass {
  pub fn new<'a>(
      storage: &'a dyn dataflow_core::system::SystemStorage,
      counters: &mut dataflow_core::system::SystemCounters,
  ) -> ThermalMass<'a> {
      ()
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