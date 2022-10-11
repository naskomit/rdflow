use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::{parse_macro_input, DeriveInput};
use syn::spanned::Spanned;
use quote::{quote};
use types::{Res};
use utils::{cerror};
use dataflow_core::reflect;

mod types;
mod utils;


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
  let field_name = field.ident.as_ref().unwrap().to_string();
  match field.ty {
    syn::Type::Path(syn::TypePath {
      path: syn::Path {ref segments, ..}, ..
    }) if segments.len() == 1 => {
      let segment = segments.first().unwrap();
      match segment.ident.to_string().as_str() {
        "Parameter" => create_param_repr(
          block_repr, field_name, segment
        )?,
        "Input" => create_input_repr(
          block_repr, field_name, segment
        )?,
        "Output" => create_output_repr(
          block_repr, field_name, segment
        )?,
        "DiscreteState" => create_discrete_state_repr(
          block_repr, field_name, segment
        )?,
        "ContinuousState" => create_continuous_state_repr(
          block_repr, field_name, segment
        )?,
        x => {
          cerror!(field, "Unknown field type {}", x)
        }
      }
    },
    _ => return Ok(())
  };

  Ok(())
}

fn get_dtype(tpe: &syn::PathSegment) -> Res<reflect::DType> {
  let type_args = match tpe.arguments {
    syn::PathArguments::AngleBracketed(
      syn::AngleBracketedGenericArguments {ref args, ..}
    ) => args,
    _ => cerror!(tpe, "Cannot parse type argumens"),
  };
  // eprintln!("{:?}", type_args);
  Ok(reflect::DType::Real)
}

fn create_param_repr(block_repr: &mut reflect::Block, name: String, segment: &syn::PathSegment) -> Res<()> {
  let dtype = get_dtype(segment)?;
  match dtype {
    reflect::DType::Real => block_repr.r_param.push(reflect::Parameter {
      name, dtype, default: None
    }),
    reflect::DType::Bool => block_repr.b_param.push(reflect::Parameter {
      name, dtype, default: None
    }),
  }
  
  Ok(())
}

fn create_input_repr(block_repr: &mut reflect::Block, name: String, segment: &syn::PathSegment) -> Res<()> {
  let dtype = get_dtype(segment)?;
  match dtype {
    reflect::DType::Real => block_repr.r_in.push(reflect::Input {
      name, dtype
    }),
    reflect::DType::Bool => block_repr.b_in.push(reflect::Input {
      name, dtype
    }),
  }
  Ok(())
}

fn create_output_repr(block_repr: &mut reflect::Block, name: String, segment: &syn::PathSegment) -> Res<()> {
  let dtype = get_dtype(segment)?;
  match dtype {
    reflect::DType::Real => block_repr.r_out.push(reflect::Output {
      name, dtype
    }),
    reflect::DType::Bool => block_repr.b_out.push(reflect::Output {
      name, dtype
    }),
  }
  Ok(())
}

fn create_discrete_state_repr(block_repr: &mut reflect::Block, name: String, segment: &syn::PathSegment) -> Res<()> {
  let dtype = get_dtype(segment)?;
  match dtype {
    reflect::DType::Bool => block_repr.b_state.push(reflect::State {
      name, dtype, initial: false
    }),
    dt => cerror!(segment, "Discrete state must be Bool (bool)")
  }
  Ok(())
}

fn create_continuous_state_repr(block_repr: &mut reflect::Block, name: String, segment: &syn::PathSegment) -> Res<()> {
  let dtype = get_dtype(segment)?;
  match dtype {
    reflect::DType::Real => block_repr.r_state.push(reflect::State {
      name, dtype, initial: 0.0
    }),
    dt => cerror!(segment, "Continuous state must be Real (f64)")
  }
  Ok(())
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