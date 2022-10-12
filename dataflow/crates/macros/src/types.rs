use syn;

pub type Res<T> = Result<T, syn::Error>;

#[derive(Default, Debug)]
pub struct SimpleType {
  pub qualified_path: Vec<syn::Ident>
}

#[derive(Default, Debug)]
pub struct GenericType {
  pub base_type: SimpleType,
  pub lifetime: Option<syn::Ident>,
  pub type_params: Vec<SimpleType>
}

impl GenericType {
  // pub fn new(base_type: &syn::Ident) -> GenericType {
  //   GenericType { base_type: base_type.to_owned(), lifetime: None, type_params: vec![] }
  // }
}