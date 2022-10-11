#[derive(Default, Debug)]
pub enum DType {
  #[default]
  Real,
  Bool
}

#[derive(Default, Debug)]
pub struct Parameter<T> {
  pub name: String,
  pub dtype: DType,
  pub default: Option<T>
}

#[derive(Default, Debug)]
pub struct Input {
  pub name: String,
  pub dtype: DType,
}

#[derive(Default, Debug)]
pub struct Output {
  pub name: String,
  pub dtype: DType,
}

#[derive(Default, Debug)]
pub struct State<T> {
  pub name: String,
  pub dtype: DType,
  pub initial: T,
}

#[derive(Default, Debug)]
pub struct Block {
  pub name: String,

  pub r_param: Vec<Parameter<f64>>,
  pub b_param: Vec<Parameter<bool>>,

  pub r_state: Vec<State<f64>>,
  pub b_state: Vec<State<bool>>,

  pub r_in: Vec<Input>,
  pub b_in: Vec<Input>,

  pub r_out: Vec<Output>,
  pub b_out: Vec<Output>,
}

impl Block {
  pub fn new(name: String) -> Block {
    Block {name, ..Block::default()}
  }
}