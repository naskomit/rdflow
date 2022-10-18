use crate::system::{SystemStorage, SystemStorageSelector, SystemStorageItemId};
use csv;

pub struct CoreWriter {
  
}

pub trait IWriter {
  type Res<T>;
  fn add_field<T: Into<SystemStorageItemId>>(&mut self, field: T);
  fn write_step(&mut self, storage: &dyn SystemStorage, time: f64) -> Self::Res<()>;
}

use std::fs::File;

pub struct CSVWriter {
  writer: csv::Writer<File>,
  selector: SystemStorageSelector
}

impl CSVWriter {
  pub fn new(path: &str) -> CSVWriter {
    let writer = csv::Writer::from_path(path).unwrap();

    CSVWriter {writer, selector: SystemStorageSelector::default()}
  }
}

impl IWriter for CSVWriter {
  type Res<T> = csv::Result<T>;
  fn add_field<T: Into<SystemStorageItemId>>(&mut self, field: T) {
    self.selector.items.push(field.into());
  }

  fn write_step(&mut self, storage: &dyn SystemStorage, time: f64) -> Self::Res<()> {
    println!("=========== Writing step ===========");
    self.writer.write_field(time.to_string())?;
    self.selector.items.iter().map(|item|
      match item {
        crate::system::SystemStorageItemId::RealParameter(ind) => {
          let value = storage.r_param_get(*ind);
          self.writer.write_field(value.to_string())
        },
        crate::system::SystemStorageItemId::BoolParameter(ind) => {
          let value = storage.b_param_get(*ind);
          self.writer.write_field(value.to_string())
        },
        crate::system::SystemStorageItemId::RealState(ind) => {
          let value = storage.r_state_get(*ind);
          self.writer.write_field(value.to_string())          
        },
        crate::system::SystemStorageItemId::BoolState(ind) => {
          let value = storage.b_state_get(*ind);
          self.writer.write_field(value.to_string())          
        },
        crate::system::SystemStorageItemId::RealOutput(ind) => {
          let value = storage.r_out_get(*ind);
          self.writer.write_field(value.to_string())          
        },
        crate::system::SystemStorageItemId::BoolOutput(ind) => {
          let value = storage.b_out_get(*ind);
          self.writer.write_field(value.to_string())          
        },
      }  
    ).collect::<csv::Result<()>>()?;
    self.writer.write_record(None::<&[u8]>)?;
    self.writer.flush()?;
    Ok(())
  }
}