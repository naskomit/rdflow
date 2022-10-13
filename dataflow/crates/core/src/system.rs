use std::marker::PhantomData;

use const_default::ConstDefault;
use const_default_derive::ConstDefault;

use crate::block::{BlockSize, IBlock, UpdateComputation};

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

    fn computations(&self) -> Vec<UpdateComputation>;
  
    fn step(&self, computations: &[UpdateComputation]) {
        for cmp in computations {
            match cmp {
                UpdateComputation::State(x) => x.f.apply(),
                UpdateComputation::Output(x) => x.f.apply(),
            }
        }
    }

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

#[macro_export]
macro_rules! static_storage {
    ($ident: ident, $($block_size: path),+) => {        
        pub mod $ident {
            use dataflow_core::system::{SystemSize, SystemStorage};
            use super::*;
            pub struct StorageFacade;

            pub const fn create_storage() -> StorageFacade {
                const SS: SystemSize = SystemSize::new()
                    $(.add($block_size))+;
                    // .add(thermal_mass::SIZE)
                    // .add(hysteresis::SIZE)
                    // .add(converter_b2f::SIZE);
              
                struct StorageImpl {
                  r_param: [f64; SS.r_param],
                  b_param: [bool; SS.b_param],
                  r_state: [f64; SS.r_state],
                  r_state_der: [f64; SS.r_state],
                  b_state: [bool; SS.b_state],
                  r_out: [f64; SS.r_out],
                  b_out: [bool; SS.b_out],
                }
              
                static mut STORAGE: StorageImpl = StorageImpl {
                  r_param: [0.0; SS.r_param],
                  b_param: [false; SS.b_param],
                  r_state: [0.0; SS.r_state],
                  r_state_der: [0.0; SS.r_state],
                  b_state: [false; SS.b_state],
                  r_out: [0.0; SS.r_out],
                  b_out: [false; SS.b_out],
                };
              
              
              
                impl SystemStorage for StorageFacade {
                  fn sizes(&self) -> SystemSize {
                    SS
                  }
              
                  fn r_param_get(&self, ind: usize) -> &f64 {
                    unsafe { &STORAGE.r_param[ind] }
                  }
                  fn r_param_set(&self, ind: usize, value: f64) {
                    unsafe { STORAGE.r_param[ind] = value }
                  }
              
                  fn b_param_get(&self, ind: usize) -> &bool {
                    unsafe {&STORAGE.b_param[ind]}
                  }
                  fn b_param_set(&self, ind: usize, value: bool) {
                    unsafe {STORAGE.b_param[ind] = value}
              
                  }
                  
                  fn r_state_get(&self, ind: usize) -> &f64 {
                    unsafe {&STORAGE.r_state[ind]}
              
                  }
                  fn r_state_set(&self, ind: usize, value: f64) {
                    unsafe {STORAGE.r_state[ind] = value}
              
                  }
                  fn r_state_der_get(&self, ind: usize) -> &f64 {
                    unsafe {&STORAGE.r_state_der[ind]}
              
                  }
                  fn r_state_der_set(&self, ind: usize, value: f64) {
                    unsafe {STORAGE.r_state_der[ind] = value}
              
                  }
                  
                  fn b_state_get(&self, ind: usize) -> &bool {
                    unsafe {&STORAGE.b_state[ind]}
              
                  }
                  fn b_state_set(&self, ind: usize, value: bool) {
                    unsafe {STORAGE.b_state[ind] = value}
              
                  }
              
                  fn r_out_get(&self, ind: usize) -> &f64 {
                    unsafe {&STORAGE.r_out[ind]}
              
                  }
                  fn r_out_set(&self, ind: usize, value: f64) {
                    unsafe {STORAGE.r_out[ind] = value}
              
                  }
                  
                  fn b_out_get(&self, ind: usize) -> &bool {
                    unsafe {&STORAGE.b_out[ind]}
              
                  }
                  fn b_out_set(&self, ind: usize, value: bool) {
                    unsafe {STORAGE.b_out[ind] = value}
              
                  }
              
                }
              
                StorageFacade
            }
            const FACADE: StorageFacade = create_storage();

            pub const fn facade() -> StorageFacade {
                FACADE
            }
        }
    };

}

pub use static_storage;