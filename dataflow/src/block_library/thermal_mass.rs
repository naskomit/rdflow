use crate::block::*;
use crate::system::{SystemStorage, SystemCounters};

pub struct Block<'a> {
  pub cp: Parameter<'a, f64>,
  pub qdot: Input<'a, f64>,
  pub t: ContinuousState<'a, f64>,
  pub t_out: Output<'a, f64>,
}

pub fn new<'a>(storage: &'a dyn SystemStorage, counters: &mut SystemCounters) -> Block<'a> {
  Block {
    cp: Parameter::new(storage, counters.next_r_param()).init(1.0),
    t: ContinuousState::new(storage, counters.next_r_state()).init(0.0),
    qdot: Input::new(storage),
    t_out: Output::new(storage, counters.next_r_out()),
  }
}

pub struct OutputUpdate {
  pub t_out: f64,
}

pub struct StateUpdate {
  pub t_dot: f64,
}

impl<'a> Block<'a> {
  pub fn outputs(&self) -> OutputUpdate {
    OutputUpdate { t_out: self.t.get()}
  }

  pub fn state_update(&self) -> StateUpdate {
    StateUpdate { t_dot: self.qdot.get() / self.cp.get() }
  }
}

pub const SIZE: BlockSize = BlockSize {
  r_param: 1,
  r_in: 1,
  r_state: 1,
  r_out: 1,
  ..BlockSize::new()
};

impl<'a> DBlock for Block<'a> {
  fn step(&mut self) {
    let state_update = self.state_update();
    self.t.der_set(state_update.t_dot);

    let output = self.outputs();
    self.t_out.set(output.t_out);
  }
}
