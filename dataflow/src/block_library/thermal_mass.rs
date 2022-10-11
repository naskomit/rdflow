use dataflow_core::block::*;
use dataflow_core::system::{SystemStorage, SystemCounters};

pub struct Block<'a> {
  pub cp: Parameter<'a, f64>,
  pub h: Parameter<'a, f64>,
  pub area: Parameter<'a, f64>,
  pub t_amb: Parameter<'a, f64>,
  pub qdot: Input<'a, f64>,
  pub t: ContinuousState<'a, f64>,
  pub t_out: Output<'a, f64>,
}

pub fn new<'a>(storage: &'a dyn SystemStorage, counters: &mut SystemCounters) -> Block<'a> {
  Block {
    cp: Parameter::<f64>::new(storage, counters.next_r_param()).init(1.0),
    h: Parameter::<f64>::new(storage, counters.next_r_param()).init(10.0),
    area: Parameter::<f64>::new(storage, counters.next_r_param()).init(1.0),
    t_amb: Parameter::<f64>::new(storage, counters.next_r_param()).init(20.0),
    t: ContinuousState::new(storage, counters.next_r_state()).init(20.0),
    qdot: Input::new(storage),
    t_out: Output::new(storage, counters.next_r_out()),
  }
}

pub struct OutputUpdate {
  pub t_out: f64,
}

pub struct StateUpdate {
  pub t_dot: f64,
  pub t: Option<f64>,
}

impl<'a> Block<'a> {
  pub fn outputs(&self) -> OutputUpdate {
    OutputUpdate { t_out: *self.t}
  }

  pub fn state_update(&self) -> StateUpdate {
    StateUpdate { 
      t_dot: (*self.qdot + *self.h * *self.area * (*self.t_amb - *self.t)) / *self.cp,
      t: None
    }
  }
}

pub const SIZE: BlockSize = BlockSize {
  r_param: 4,
  r_in: 1,
  r_state: 1,
  r_out: 1,
  ..BlockSize::new()
};

impl<'a> IBlock for Block<'a> {
  fn step(&self) {
    let state_update = self.state_update();
    self.t.der_set(state_update.t_dot);

    let output = self.outputs();
    self.t_out.set(output.t_out);
  }
}
