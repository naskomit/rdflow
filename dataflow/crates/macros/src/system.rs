use proc_macro2::TokenStream as TokenStream2;
use syn::{DeriveInput};
use syn::spanned::Spanned;
use quote::{quote};
use dataflow_reflect::system as rsystem;
// use dataflow_reflect::block::AttachTo;
use std::marker::PhantomData;
use std::str::FromStr;
use crate::types::{Res, SimpleType};
use crate::utils::{cerror, lerror};
use crate::parsers;



pub fn create_system_repr(ast: &DeriveInput) -> Res<rsystem::System> {
  Ok(rsystem::System {

  })
}

pub fn generate_storage(system_repr: &rsystem::System) -> TokenStream2 {
  let ident = "Thermal";

  

  quote!{
    pub mod #ident {
      use dataflow_core::system::{SystemSize, SystemStorage};
      use super::*;
      pub struct StorageFacade;

      pub const fn create_storage() -> StorageFacade {
        const SS: SystemSize = SystemSize::new()
          $(.add($block_size))+;
        
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
  }
}