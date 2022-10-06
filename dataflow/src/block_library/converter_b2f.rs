use crate::block::{Parameter, Input, Output, DBlock, BlockSize, Access, ReadAccess, Initial};
use crate::system::{SystemStorage, SystemCounters};

pub struct Block<'a> {
  pub true_value: Parameter<'a, f64>,
  pub false_value: Parameter<'a, f64>,
  pub in1: Input<'a, bool>,
  pub out1: Output<'a, f64>,
}

pub fn new<'a>(storage: &'a dyn SystemStorage, counters: &mut SystemCounters) -> Block<'a> {
  Block {
    true_value: Parameter::new(storage, counters.next_r_param()).init(1.0),
    false_value: Parameter::new(storage, counters.next_r_param()).init(0.0),
    in1: Input::new(storage),
    out1: Output::new(storage, counters.next_r_out()),
  }
}

pub struct OutputUpdate {
  pub out1: f64
}

impl<'a> Block<'a> {
  pub fn outputs(&self) -> OutputUpdate {
    OutputUpdate { out1: 
      if self.in1.get() {
        self.true_value.get() } else {
          self.false_value.get()
        }
    }
  }

}

pub const SIZE: BlockSize = BlockSize {
  r_param: 2,
  b_in: 1,
  r_out: 1,
  ..BlockSize::new()
};

impl<'a> DBlock for Block<'a> {
  fn step(&mut self) {
    let output = self.outputs();
    self.out1.set(output.out1);
  }
}