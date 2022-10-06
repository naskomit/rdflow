use crate::block::*;
use crate::system::{SystemStorage, SystemCounters};

pub struct Block<'a> {
  pub low_threshold: Parameter<'a, f64>,
  pub high_treshold: Parameter<'a, f64>,
  pub out_inverted: Parameter<'a, bool>,
  pub in1: Input<'a, f64>,
  pub out1: Output<'a, bool>,
  pub state_high: DiscreteState<'a, bool>
}

pub fn new<'a>(storage: &'a dyn SystemStorage, counters: &mut SystemCounters) -> Block<'a> {
  Block {
    low_threshold: Parameter::new(storage, counters.next_r_param()).init(0.0),
    high_treshold: Parameter::new(storage, counters.next_r_param()).init(1.0),
    out_inverted: Parameter::new(storage, counters.next_b_param()).init(false),
    in1: Input::new(storage),
    out1: Output::new(storage, counters.next_b_out()),
    state_high: DiscreteState::new(storage, counters.next_b_state()).init(false),
  }
} 

pub struct OutputUpdate {
  pub out1: bool,
}

pub struct StateUpdate {
  pub state1: Option<bool>,
}

impl<'a> Block<'a> {
  pub fn outputs(&self) -> OutputUpdate {
    OutputUpdate { out1: 
      if self.out_inverted.get() {
        self.state_high.get() } else {
          self.state_high.get()
        }
    }
  }

  pub fn state_update(&self) -> StateUpdate {
    let state1_new = 
      if self.in1.get() < self.low_threshold.get() && self.state_high.get() == false {
          Some(false)
      } else if self.in1.get() > self.high_treshold.get() && self.state_high.get() == true {
          Some(true)
      } else {
        None
      };
    StateUpdate { state1: state1_new }
  }
}


pub const SIZE: BlockSize = BlockSize {
  r_param: 2,
  b_param: 1,
  r_in: 1,
  b_out: 1,
  b_state: 1,
  ..BlockSize::new()
};

impl<'a> DBlock for Block<'a> {
  fn step(&mut self) {
    let state_update = self.state_update();
    match state_update.state1 {
      Some(x) => self.state_high.set(x),
      None => ()
    }

    let output = self.outputs();
    self.out1.set(output.out1);
  }
}