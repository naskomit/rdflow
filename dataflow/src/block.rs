use std::{marker::PhantomData};
use const_default::ConstDefault;
use super::system::{SystemStorage};


pub trait ReadAccess<T> {
  fn get(&self) -> T;
}

pub trait Access<T> {
  fn get(&self) -> T;
  fn set(&mut self, v: T);
}

pub trait DerivativeAccess<T> {
  fn der_get(&self) -> T;
  fn der_set(&mut self, v: T);
}

pub trait Initial<T> {
  fn init(self, v: T) -> Self;
}

/** Parameters */
pub struct Parameter<'a, T: Copy> {
  storage: &'a dyn SystemStorage,
  id: usize,
  _marker: PhantomData<&'a T>,
}


impl<'a, T: Copy> Parameter<'a, T> {  
  pub const fn new(storage: &'a dyn SystemStorage, id: usize) -> Parameter<'a, T> {
    Parameter::<'a, T> { 
      storage: storage, id: id, _marker: PhantomData {}
    }
  }
}

impl<'a> Access<f64> for Parameter<'a, f64> {
  fn get(&self) -> f64 {
    self.storage.r_param_get(self.id)
  }
  fn set(&mut self, v: f64) {
    self.storage.r_param_set(self.id, v)
  }
}

impl<'a> Initial<f64> for Parameter<'a, f64> {
  fn init(mut self, v: f64) -> Self {
    Access::set(&mut self, v);
    self
  }
}

impl<'a> Access<bool> for Parameter<'a, bool> {
  fn get(&self) -> bool {
    self.storage.b_param_get(self.id)
  }
  fn set(&mut self, v: bool) {
    self.storage.b_out_set(self.id, v)
  }
}

impl<'a> Initial<bool> for Parameter<'a, bool> {
  fn init(mut self, v: bool) -> Self {
    Access::set(&mut self, v);
    self
  }
}

/** Inputs & Outputs */
pub struct Input<'a, T: Copy> {
  storage: &'a dyn SystemStorage,
  output_id: Option<usize>,
  _marker: PhantomData<&'a T>
}

impl<'a, T: Copy> Input<'a, T> {  
  pub const fn new(storage: &'a dyn SystemStorage) -> Input<'a, T> {
    Input::<'a, T> { 
      storage: storage, output_id: None, _marker:  PhantomData {}
    }
  }

  pub fn connect(&mut self, output: &Output<'a, T>) {
    self.output_id = Some(output.id);
  } 
}

impl<'a> ReadAccess<f64> for Input<'a, f64> {
  fn get(&self) -> f64 {
    self.storage.r_out_get(self.output_id.unwrap())
  }
}

impl<'a> ReadAccess<bool> for Input<'a, bool> {
  fn get(&self) -> bool {
    self.storage.b_out_get(self.output_id.unwrap())
  }
}

pub struct Output<'a, T: Copy> {
  storage: &'a dyn SystemStorage,
  id: usize,
  _marker: PhantomData<&'a T>
}

impl<'a, T: Copy> Output<'a, T> {  
  pub const fn new(storage: &'a dyn SystemStorage, id: usize) -> Output<'a, T> {
    Output::<'a, T> { 
      storage: storage, id: id, _marker:  PhantomData {}
    }
  }
}

impl<'a> Access<f64> for Output<'a, f64> {
  fn get(&self) -> f64 {
    self.storage.r_out_get(self.id)
  }
  fn set(&mut self, v: f64) {
    self.storage.r_out_set(self.id, v)
  }
}

impl<'a> Access<bool> for Output<'a, bool> {
  fn get(&self) -> bool {
    self.storage.b_out_get(self.id)
  }
  fn set(&mut self, v: bool) {
    self.storage.b_out_set(self.id, v)
  }
}



/** States */
pub struct DiscreteState<'a, T: Copy> {
  storage: &'a dyn SystemStorage,
  id: usize,
  _marker: PhantomData<&'a T>
}

impl<'a, T: Copy> DiscreteState<'a, T> {  
  pub const fn new(storage: &'a dyn SystemStorage, id: usize) -> DiscreteState<'a, T> {
    DiscreteState::<'a, T> { 
      storage: storage, id: id, _marker:  PhantomData {}
    }
  }
}

impl<'a> Access<bool> for DiscreteState<'a, bool> {
  fn get(&self) -> bool {
    self.storage.b_state_get(self.id)
  }
  fn set(&mut self, v: bool) {
    self.storage.b_state_set(self.id, v)
  }
}

impl<'a> Initial<bool> for DiscreteState<'a, bool> {
  fn init(mut self, v: bool) -> Self {
    Access::set(&mut self, v);
    self
  }
}

pub struct ContinuousState<'a, T: Copy> {
  storage: &'a dyn SystemStorage,
  id: usize,
  _marker: PhantomData<&'a T>
}

impl<'a, T: Copy> ContinuousState<'a, T> {  
  pub const fn new(storage: &'a dyn SystemStorage, id: usize) -> ContinuousState<'a, T> {
    ContinuousState::<'a, T> { 
      storage: storage, id: id, _marker:  PhantomData {}
    }
  }
}

impl<'a> Access<f64> for ContinuousState<'a, f64> {
  fn get(&self) -> f64 {
    self.storage.r_state_get(self.id)
  }
  fn set(&mut self, v: f64) {
    self.storage.r_state_set(self.id, v)
  }
}

impl<'a> DerivativeAccess<f64> for ContinuousState<'a, f64> {
  fn der_get(&self) -> f64 {
    self.storage.r_state_der_get(self.id)
  }
  fn der_set(&mut self, v: f64) {
    self.storage.r_state_der_set(self.id, v)
  }
}

impl<'a> Initial<f64> for ContinuousState<'a, f64> {
  fn init(mut self, v: f64) -> Self {
    Access::set(&mut self, v);
    self
  }
}

enum BlockCausality {
  Functional,
  State,
  Mixed
}

#[derive(const_default_derive::ConstDefault)]
pub struct BlockSize {
  pub r_param: usize,
  pub b_param: usize,

  pub r_state: usize,
  pub b_state: usize,

  pub r_in: usize,
  pub b_in: usize,

  pub r_out: usize,
  pub b_out: usize,
}

impl BlockSize {
  pub const fn new() -> BlockSize {
    <BlockSize as ConstDefault>::DEFAULT
  }
}


pub trait DBlock {
  // const SIZE: BlockSize;
  // fn size() -> &'static BlockSize;
  fn step(&mut self);
}