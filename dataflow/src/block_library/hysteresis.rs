use dataflow_core::block::*;
use dataflow_core::system::{SystemStorage, SystemCounters};

pub struct Block<'a> {
  pub low_threshold: Parameter<'a, f64>,
  pub high_threshold: Parameter<'a, f64>,
  pub out_inverted: Parameter<'a, bool>,
  pub in1: Input<'a, f64>,
  pub out1: Output<'a, bool>,
  pub state_high: DiscreteState<'a, bool>
}

pub fn new<'a>(storage: &'a dyn SystemStorage, counters: &mut SystemCounters) -> Block<'a> {
  Block {
    low_threshold: Parameter::<f64>::new(storage, counters.next_r_param()).init(0.0),
    high_threshold: Parameter::<f64>::new(storage, counters.next_r_param()).init(1.0),
    out_inverted: Parameter::<bool>::new(storage, counters.next_b_param()).init(false),
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
      if *self.out_inverted {
        !*self.state_high
      } else {
        *self.state_high
      }
    }
  }

  pub fn state_update(&self) -> StateUpdate {
    let state1_new = 
      if *self.in1 < *self.low_threshold && *self.state_high {
          Some(false)
      } else if *self.in1 > *self.high_threshold && !*self.state_high {
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

impl<'a> IBlock for Block<'a> {
  fn step(&self) {
    let state_update = self.state_update();
    match state_update.state1 {
      Some(x) => self.state_high.set(x),
      None => ()
    }

    let output = self.outputs();
    self.out1.set(output.out1);
  }
}