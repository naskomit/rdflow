use dataflow_core::system::{SystemSize, SystemStorage, ISystem, SystemCounters, static_storage};
use dataflow_core::block::{IBlock, Access, DerivativeAccess, BlockComputation, UpdateComputation};
use crate::block_library::{hysteresis, thermal_mass, converter_b2f};

static_storage!(Storage, 
  thermal_mass::SIZE,
  hysteresis::SIZE, 
  converter_b2f::SIZE
);

pub struct Blocks<'a> {
  pub thermal_mass: thermal_mass::Block<'a>,
  pub hyst_component: hysteresis::Block<'a>,
  pub b2f: converter_b2f::Block<'a>
}

pub struct SystemImpl<'a> {
  pub storage: Storage::StorageFacade,
  pub components: Blocks<'a>,
}

impl<'a> SystemImpl<'a> {


  pub fn new() -> SystemImpl<'a> {
    const STORAGE: Storage::StorageFacade = Storage::facade();

    let mut counters: SystemCounters = SystemCounters::new();

    let components: Blocks = Blocks {
      thermal_mass: thermal_mass::new(&STORAGE, &mut counters),
      hyst_component: hysteresis::new(&STORAGE, &mut counters),  
      b2f: converter_b2f::new(&STORAGE, &mut counters),
    };
    
    println!("{:?}", counters);
    
    let mut instance = SystemImpl {
      storage: STORAGE, 
      components: components,
    };

    instance.connect();
    instance.init();
    instance

  }

  pub fn connect(&mut self) {
    let components = &mut self.components;
    components.hyst_component.in1.connect(&components.thermal_mass.t_out);
    components.b2f.in1.connect(&components.hyst_component.out1);
    components.thermal_mass.qdot.connect(&components.b2f.out1);
  }

  pub fn init(&self) {
    self.components.thermal_mass.cp.set(4000.0);
    self.components.thermal_mass.area.set(1.0);
    
    self.components.hyst_component.low_threshold.set(30.0);
    self.components.hyst_component.high_threshold.set(32.0);
    self.components.hyst_component.out_inverted.set(true);

    self.components.b2f.true_value.set(500.0);


  }

}


impl<'a> ISystem<'a> for SystemImpl<'a> {
  const N_BLOCKS: usize = 3;

  fn storage(&self) -> &dyn SystemStorage {
    &self.storage
  }

  fn block(&'a self, i: usize) -> Option<&'a dyn IBlock> {
    match i {
      0 => Some(&self.components.hyst_component),
      1 => Some(&self.components.b2f),
      2 => Some(&self.components.thermal_mass),
      _ => None
    }    
  }

  fn computations(&self) -> Vec<UpdateComputation> {
    let comp_therm_mass = match self.components.thermal_mass.get_computation() {
      BlockComputation::State(x) => x,
      _ => panic!()
    };
    let comp_hyst = match self.components.hyst_component.get_computation() {
      BlockComputation::Mixed(x) => x,
      _ => panic!()
    };
    let comp_b2f = match self.components.b2f.get_computation() {
      BlockComputation::Functional(x) => x,
      _ => panic!()
    };


    vec![
        UpdateComputation::Output(comp_therm_mass.output_update_fn.to_owned()),
        UpdateComputation::State(comp_hyst.state_update_fn.to_owned()),
        UpdateComputation::Output(comp_hyst.output_update_fn.to_owned()),
        UpdateComputation::Output(comp_b2f.output_update_fn.to_owned()),
        UpdateComputation::State(comp_therm_mass.state_update_fn.to_owned()),
    ]
  }

}
