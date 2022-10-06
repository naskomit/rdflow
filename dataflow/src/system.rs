use const_default::ConstDefault;
use const_default_derive::ConstDefault;

use crate::block::{BlockSize};

#[allow(dead_code)]
#[derive(Default, Debug, ConstDefault, PartialEq, Eq)]
pub struct SystemSize {
    pub r_param: usize,
    pub b_param: usize,
    pub r_state: usize,
    pub b_state: usize,
    pub r_out: usize,
    pub b_out: usize,
}

#[allow(dead_code)]
impl SystemSize {
    pub const fn new() -> SystemSize {
        SystemSize::DEFAULT
    }


    pub const fn add(&self, bs: BlockSize) -> SystemSize {
        SystemSize { 
            r_param: self.r_param + bs.r_param,
            b_param: self.b_param + bs.b_param,
            r_state: self.r_state + bs.r_state,
            b_state: self.b_state + bs.b_state,
            r_out: self.r_out + bs.r_out,
            b_out: self.b_out + bs.b_out,
        }
    }
}

pub trait SystemStorage : Sync {
    fn r_param_get(&self, ind: usize) -> f64;
    fn r_param_set(&self, ind: usize, value: f64);
    
    fn b_param_get(&self, ind: usize) -> bool;
    fn b_param_set(&self, ind: usize, value: bool);
    
    
    fn r_state_get(&self, ind: usize) -> f64;
    fn r_state_set(&self, ind: usize, value: f64);
    fn r_state_der_get(&self, ind: usize) -> f64;
    fn r_state_der_set(&self, ind: usize, value: f64);
    
    fn b_state_get(&self, ind: usize) -> bool;
    fn b_state_set(&self, ind: usize, value: bool);

    fn r_out_get(&self, ind: usize) -> f64;
    fn r_out_set(&self, ind: usize, value: f64);
    
    fn b_out_get(&self, ind: usize) -> bool;
    fn b_out_set(&self, ind: usize, value: bool);

}

#[derive(ConstDefault, Debug)]
pub struct SystemCounters {
    next_r_param: usize,
    next_b_param: usize,
    next_r_state: usize,
    next_b_state: usize,
    next_r_out: usize,
    next_b_out: usize
}


impl SystemCounters {
    pub const fn new() -> SystemCounters {
        SystemCounters::DEFAULT
    }

    pub fn next_r_param(&mut self) -> usize {
        let res = self.next_r_param;
        self.next_r_param += 1;
        res
    }
    pub fn next_b_param(&mut self) -> usize {
        let res = self.next_b_param;
        self.next_b_param += 1;
        res
    }
    pub fn next_r_state(&mut self) -> usize {
        let res = self.next_r_state;
        self.next_r_state += 1;
        res
    }
    pub fn next_b_state(&mut self) -> usize {
        let res = self.next_b_state;
        self.next_b_state += 1;
        res
    }
    pub fn next_r_out(&mut self) -> usize {
        let res = self.next_r_out;
        self.next_r_out += 1;
        res
    }
    pub fn next_b_out(&mut self) -> usize {
        let res = self.next_b_out;
        self.next_b_out += 1;
        res
    }
  }

pub trait System {
    fn step(&mut self);
}