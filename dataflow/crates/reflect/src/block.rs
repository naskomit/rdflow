use std::str::FromStr;
use std::marker::PhantomData;
use proc_macro2::Span;

#[derive(Default, Debug, Clone)]
pub enum DType {
  #[default]
  Real,
  Bool
}

#[derive(Default, Debug, Clone)]
pub enum BlockFieldType {
  #[default]
  Parameter,
  Input,
  Output,
  ContinuousState,
  DiscreteState,
}

impl FromStr for BlockFieldType {
    type Err = String;
    fn from_str(x: &str) -> Result<BlockFieldType, Self::Err> {
      match x {
        "Parameter" => Ok(BlockFieldType::Parameter),
        "Input" => Ok(BlockFieldType::Input),
        "Output" => Ok(BlockFieldType::Output),
        "DiscreteState" => Ok(BlockFieldType::DiscreteState),
        "ContinuousState" => Ok(BlockFieldType::ContinuousState),
        x => {
          Err(format!("Unknown field btype {}", x))
        },      }
    }
}

#[derive(Default, Debug)]
pub struct BlockField {
  pub name: String,
  pub span: Option<Span>,
  pub dtype: DType,
  pub btype: BlockFieldType
}


#[derive(Default, Debug)]
pub struct Parameter<T> {
  pub field: BlockField,
  pub default: Option<T>
}

#[derive(Default, Debug)]
pub struct Input<T> {
  pub field: BlockField,
  pub _marker: PhantomData<T>,
}

#[derive(Default, Debug)]
pub struct Output<T> {
  pub field: BlockField,
  pub _marker: PhantomData<T>,
}

#[derive(Default, Debug)]
pub struct State<T> {
  pub field: BlockField,
  pub initial: T,
}

#[derive(Default, Debug)]
pub struct Block {
  pub name: String,
  pub span: Option<Span>,

  pub r_param: Vec<Parameter<f64>>,
  pub b_param: Vec<Parameter<bool>>,

  pub r_state: Vec<State<f64>>,
  pub b_state: Vec<State<bool>>,

  pub r_in: Vec<Input<f64>>,
  pub b_in: Vec<Input<bool>>,

  pub r_out: Vec<Output<f64>>,
  pub b_out: Vec<Output<bool>>,
}

impl Block {
  pub fn new(name: String) -> Block {
    Block {name, ..Block::default()}
  }
}

pub trait AttachTo {
  fn attach_to(self, block: &mut Block);
}

macro_rules! attach_impl {
  ($ty: ty, $dest: ident) => {
    impl AttachTo for $ty {
        fn attach_to (self, block: &mut Block) {
            block.$dest.push(self);
        }
    }
    };
}


attach_impl!(Parameter<f64>, r_param);
attach_impl!(Parameter<bool>, b_param);
attach_impl!(Input<f64>, r_in);
attach_impl!(Input<bool>, b_in);
attach_impl!(Output<f64>, r_out);
attach_impl!(Output<bool>, b_out);
attach_impl!(State<f64>, r_state);
attach_impl!(State<bool>, b_state);
