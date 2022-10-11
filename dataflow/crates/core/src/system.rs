use std::marker::PhantomData;

use const_default::ConstDefault;
use const_default_derive::ConstDefault;

use crate::block::{BlockSize, IBlock};

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
    fn sizes(&self) -> SystemSize;

    fn r_param_get(&self, ind: usize) -> &f64;
    fn r_param_set(&self, ind: usize, value: f64);
    
    fn b_param_get(&self, ind: usize) -> &bool;
    fn b_param_set(&self, ind: usize, value: bool);
    
    
    fn r_state_get(&self, ind: usize) -> &f64;
    fn r_state_set(&self, ind: usize, value: f64);
    fn r_state_der_get(&self, ind: usize) -> &f64;
    fn r_state_der_set(&self, ind: usize, value: f64);
    
    fn b_state_get(&self, ind: usize) -> &bool;
    fn b_state_set(&self, ind: usize, value: bool);

    fn r_out_get(&self, ind: usize) -> &f64;
    fn r_out_set(&self, ind: usize, value: f64);
    
    fn b_out_get(&self, ind: usize) -> &bool;
    fn b_out_set(&self, ind: usize, value: bool);

    fn print_params(&self) {
        print!("r_param: ");
        for i in 0..self.sizes().r_param {
            print!("{},", self.r_param_get(i));
        }
        println!("");

        print!("b_param: ");
        for i in 0..self.sizes().b_param {
            print!("{},", self.b_param_get(i));
        }
        println!("");
    }

    fn print_states_outputs(&self) {
        print!("r_state: ");
        for i in 0..self.sizes().r_state {
            print!("{} ({}),", self.r_state_get(i), self.r_state_der_get(i));
        }
        println!("");

        print!("b_state: ");
        for i in 0..self.sizes().b_state {
            print!("{},", self.b_state_get(i));
        }
        println!("");

        print!("r_out: ");
        for i in 0..self.sizes().r_out {
            print!("{},", self.r_out_get(i));
        }
        println!("");

        print!("b_out: ");
        for i in 0..self.sizes().b_out {
            print!("{},", self.b_out_get(i));
        }
        println!("");
    }

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


pub struct BlockIterator<'a, T: ?Sized> {
    pub system: &'a T,
    pub current: usize,
    _marker: PhantomData<T>
}

impl<'a, T: ?Sized> BlockIterator<'a, T> {
    pub fn new(system: &'a T) -> BlockIterator<'a, T> {
        BlockIterator::<'a, T> {
            system: system,
            current: 0,
            _marker: std::marker::PhantomData,
        }    
    }
}

impl<'a, S: ISystem<'a>> Iterator for BlockIterator<'a, S> {
    type Item = &'a dyn IBlock;
  
    fn next(&mut self) -> Option<Self::Item> {
        let block = self.system.block(self.current);
        match block {
            Some(x) => {self.current += 1; Some(x)}
            None => None
        }    
    }
  
}

pub trait ISystem<'a> : Sized {
    const N_BLOCKS: usize;

    fn storage(&self) -> &dyn SystemStorage;

    fn block(&'a self, i: usize) -> Option<&'a dyn IBlock>;

    fn blocks(&self)-> BlockIterator<Self> {
        BlockIterator::<Self>::new(self)
    }
  
    fn step(&'a self);
    //  {
    //     // for i in 0..Self::N_BLOCKS {
    //     //     self.block(i).unwrap().step();
    //     // }
    //     for block in self.blocks() {
    //         block.step();
    //     }
    // }

    fn advance_continuous_state(&self, dt: f64) {
        let storage = self.storage();
        for i in 0..(storage.sizes().r_state) {
            let old_value = storage.r_state_get(i);
            let der = storage.r_state_der_get(i);
            
            // NOTE: This could be a different integrator
            let new_value = old_value + der * dt;

            storage.r_state_set(i, new_value);
        }
    }
}