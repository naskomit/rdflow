use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::{parse_macro_input, DeriveInput, PathSegment};
use syn::spanned::Spanned;
use quote::{quote};
use types::{Res, GenericType, SimpleType};
use utils::{cerror, lerror};
use dataflow_core::reflect::{self, BlockField, BlockFieldType, AttachTo};
use std::str::FromStr;
use std::marker::PhantomData;

mod types;
mod utils;
mod parsers;


#[proc_macro_derive(Block, attributes(default, initial))]
pub fn derive(input: TokenStream) -> TokenStream {
  let ast = parse_macro_input!(input as DeriveInput);
  let block_repr = create_block_repr(&ast);
  let output = block_repr.and_then(|repr| {
    let size_section = create_size(&repr);
    Ok(quote!{
      #size_section
    })  
  });

  match output {
    Ok(res) => res.into(),
    Err(e) => e.to_compile_error().into()
  }
}

fn create_block_repr(ast: &DeriveInput) -> Res<reflect::Block> {
  let block_name = &ast.ident.to_string();
  let mut block_repr = reflect::Block::new(block_name.clone());
  let fields = match ast.data {
    syn::Data::Struct(syn::DataStruct{
      fields: syn::Fields::Named(syn::FieldsNamed {
        ref named, ..
      }), ..
    }) => named,
    _ => cerror!(ast, "Block definition must be a `struct` with named fields")
  };

  for field in fields.iter() {
    create_field_repr(&mut block_repr, field)?;
  }

  eprintln!("{:#?}", block_repr);
  Ok(block_repr)
}

fn create_field_repr(block_repr: &mut reflect::Block, field: &syn::Field) -> Res<()> {
  let field_ident = field.ident.as_ref().unwrap();
  let field_type = parsers::parse_qualified_generic_type(&field.ty)?;
  let btype = match BlockFieldType::from_str(
    field_type.base_type.qualified_path.last().unwrap().to_string().as_str()
  ) {
    Ok(bt) => bt,
    Err(err) => cerror!(field, "{}", err),
  };

  let dtype = if field_type.type_params.len() == 1 {
    get_dtype(field_type.type_params.first().unwrap())?
  } else {
    cerror!(field, "too many type parameters ({}) for field {}",
      field_type.type_params.len(), field_ident
    )
  };
  let block_field = BlockField {
    name: field_ident.to_string(), 
    dtype: dtype.clone(), btype: btype.clone()
  };
  eprintln!("{:?}", block_field);
  match btype {
    BlockFieldType::Parameter => match dtype {
      reflect::DType::Real => reflect::Parameter::<f64> {
        field: block_field, default: Some(0.0)
      }.attach_to(block_repr),
      reflect::DType::Bool => reflect::Parameter::<bool> {
        field: block_field, default: Some(false)
      }.attach_to(block_repr),
    },
    BlockFieldType::Input => match dtype {
      reflect::DType::Real => reflect::Input::<f64> {
        field: block_field, _marker: PhantomData
      }.attach_to(block_repr), 
      reflect::DType::Bool => reflect::Input::<bool> {
        field: block_field, _marker: PhantomData
      }.attach_to(block_repr), 
    },
    BlockFieldType::Output => match dtype {
      reflect::DType::Real => reflect::Output::<f64> {
        field: block_field, _marker: PhantomData
      }.attach_to(block_repr),
      reflect::DType::Bool => reflect::Output::<bool> {
        field: block_field, _marker: PhantomData
      }.attach_to(block_repr),
    },
    BlockFieldType::DiscreteState => match dtype {
      reflect::DType::Real => cerror!(
        field, "Discrete state cannot be of type f64"
      ),
      reflect::DType::Bool => reflect::State::<bool> {
        field: block_field, initial: false
      }.attach_to(block_repr),
    },
    BlockFieldType::ContinuousState => match dtype {
      reflect::DType::Real => reflect::State::<f64> {
        field: block_field, initial: 0.0
      }.attach_to(block_repr),
      reflect::DType::Bool => cerror!(
        field, "Continuous state should be of type f64"
      ),
    },
    x => {
      cerror!(field, "Unknown field type {:?}", x)
    },
  };

  Ok(())
}

fn get_dtype(tpe: &SimpleType) -> Res<reflect::DType> {
  let dtype_ident = tpe.qualified_path.last().unwrap();
  match dtype_ident.to_string().as_str() {
    "f64" => Ok(reflect::DType::Real),
    "bool" => Ok(reflect::DType::Bool),
    x => cerror!(dtype_ident, "Unknown field dtype {}", dtype_ident.to_string())
  }
}




fn create_size(block_repr: &reflect::Block) -> TokenStream2 {
  // 

  let r_param = block_repr.r_param.len();
  let b_param = block_repr.b_param.len();

  let r_state = block_repr.r_state.len();
  let b_state = block_repr.b_state.len();

  let r_in = block_repr.r_in.len();
  let b_in = block_repr.b_in.len();

  let r_out = block_repr.r_out.len();
  let b_out = block_repr.b_out.len();

  quote!{
    pub const SIZE: dataflow_core::block::BlockSize = dataflow_core::block::BlockSize {
      r_param: #r_param,
      b_param: #b_param,
    
      r_state: #r_state,
      b_state: #b_state,
    
      r_in: #r_in,
      b_in: #b_in,
    
      r_out: #r_out,
      b_out: #b_out,    
    };
  }


}