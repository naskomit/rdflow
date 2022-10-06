use crate::system::{SystemSize, SystemStorage, ISystem, SystemCounters};
use crate::block_library::{hysteresis, thermal_mass, converter_b2f};
use crate::block::{IBlock};

pub struct StorageFacade;

pub struct Components<'a> {
  thermal_mass: thermal_mass::Block<'a>,
  hyst_component: hysteresis::Block<'a>,
  b2f: converter_b2f::Block<'a>
}

pub struct SystemImpl<'a> {
  pub storage: StorageFacade,
  pub components: Components<'a>,
}

pub const fn create_storage() -> StorageFacade {
  const SS: SystemSize = SystemSize::new()
  .add(thermal_mass::SIZE)
  .add(hysteresis::SIZE)
  .add(converter_b2f::SIZE);

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
    fn r_param_get(&self, ind: usize) -> f64 {
      unsafe { STORAGE.r_param[ind] }
    }
    fn r_param_set(&self, ind: usize, value: f64) {
      unsafe { STORAGE.r_param[ind] = value }
    }

    fn b_param_get(&self, ind: usize) -> bool {
      unsafe {STORAGE.b_param[ind]}
    }
    fn b_param_set(&self, ind: usize, value: bool) {
      unsafe {STORAGE.b_param[ind] = value}

    }
    
    fn r_state_get(&self, ind: usize) -> f64 {
      unsafe {STORAGE.r_state[ind]}

    }
    fn r_state_set(&self, ind: usize, value: f64) {
      unsafe {STORAGE.r_state[ind] = value}

    }
    fn r_state_der_get(&self, ind: usize) -> f64 {
      unsafe {STORAGE.r_state_der[ind]}

    }
    fn r_state_der_set(&self, ind: usize, value: f64) {
      unsafe {STORAGE.r_state_der[ind] = value}

    }
    
    fn b_state_get(&self, ind: usize) -> bool {
      unsafe {STORAGE.b_state[ind]}

    }
    fn b_state_set(&self, ind: usize, value: bool) {
      unsafe {STORAGE.b_state[ind] = value}

    }

    fn r_out_get(&self, ind: usize) -> f64 {
      unsafe {STORAGE.r_out[ind]}

    }
    fn r_out_set(&self, ind: usize, value: f64) {
      unsafe {STORAGE.r_out[ind] = value}

    }
    
    fn b_out_get(&self, ind: usize) -> bool {
      unsafe {STORAGE.b_out[ind]}

    }
    fn b_out_set(&self, ind: usize, value: bool) {
      unsafe {STORAGE.b_out[ind] = value}

    }

  }

  StorageFacade
}

impl<'a> SystemImpl<'a> {


  pub fn new() -> SystemImpl<'a> {
    const STORAGE: StorageFacade = create_storage();
    let mut counters: SystemCounters = SystemCounters::new();

    let mut components: Components = Components {
      thermal_mass: thermal_mass::new(&STORAGE, &mut counters),
      hyst_component: hysteresis::new(&STORAGE, &mut counters),  
      b2f: converter_b2f::new(&STORAGE, &mut counters),
    };

    components.hyst_component.in1.connect(&components.thermal_mass.t_out);
    components.b2f.in1.connect(&components.hyst_component.out1);
    components.thermal_mass.qdot.connect(&components.b2f.out1);

    println!("{:?}", counters);

    SystemImpl {storage: STORAGE, components: components}
  }
}

impl<'a> ISystem for SystemImpl<'a> {
    fn step(&mut self) {
        self.components.hyst_component.step();
        self.components.b2f.step();
        self.components.thermal_mass.step();
    }
}
