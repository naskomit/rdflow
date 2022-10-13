use proc_macro2::TokenStream as TokenStream2;
use syn::{DeriveInput};
use syn::spanned::Spanned;
use quote::{quote};
use dataflow_reflect::block as rblock;
use dataflow_reflect::block::AttachTo;
use std::marker::PhantomData;
use std::str::FromStr;
use crate::types::{Res, SimpleType};
use crate::utils::{cerror, lerror};
use crate::parsers;


/** Helpers */
pub fn create_block_repr(ast: &DeriveInput) -> Res<rblock::Block> {
  let block_name = &ast.ident.to_string();
  let mut block_repr = rblock::Block::new(block_name.clone());
  block_repr.span = Some(ast.span());
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

  // eprintln!("{:#?}", block_repr);
  Ok(block_repr)
}

fn create_field_repr(block_repr: &mut rblock::Block, field: &syn::Field) -> Res<()> {
  let field_ident = field.ident.as_ref().unwrap();
  let field_type = parsers::parse_qualified_generic_type(&field.ty)?;
  let btype = match rblock::BlockFieldType::from_str(
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
  let block_field = rblock::BlockField {
    name: field_ident.to_string(),
    span: Some(field.span()),
    dtype: dtype.clone(), btype: btype.clone()
  };
  // eprintln!("{:?}", block_field);
  match btype {
    rblock::BlockFieldType::Parameter => match dtype {
      rblock::DType::Real => rblock::Parameter::<f64> {
        field: block_field, default: Some(0.0)
      }.attach_to(block_repr),
      rblock::DType::Bool => rblock::Parameter::<bool> {
        field: block_field, default: Some(false)
      }.attach_to(block_repr),
    },
    rblock::BlockFieldType::Input => match dtype {
      rblock::DType::Real => rblock::Input::<f64> {
        field: block_field, _marker: PhantomData
      }.attach_to(block_repr), 
      rblock::DType::Bool => rblock::Input::<bool> {
        field: block_field, _marker: PhantomData
      }.attach_to(block_repr), 
    },
    rblock::BlockFieldType::Output => match dtype {
      rblock::DType::Real => rblock::Output::<f64> {
        field: block_field, _marker: PhantomData
      }.attach_to(block_repr),
      rblock::DType::Bool => rblock::Output::<bool> {
        field: block_field, _marker: PhantomData
      }.attach_to(block_repr),
    },
    rblock::BlockFieldType::DiscreteState => match dtype {
      rblock::DType::Real => cerror!(
        field, "Discrete state cannot be of type f64"
      ),
      rblock::DType::Bool => rblock::State::<bool> {
        field: block_field, initial: false
      }.attach_to(block_repr),
    },
    rblock::BlockFieldType::ContinuousState => match dtype {
      rblock::DType::Real => rblock::State::<f64> {
        field: block_field, initial: 0.0
      }.attach_to(block_repr),
      rblock::DType::Bool => cerror!(
        field, "Continuous state should be of type f64"
      ),
    },
  };

  Ok(())
}

fn get_dtype(tpe: &SimpleType) -> Res<rblock::DType> {
  let dtype_ident = tpe.qualified_path.last().unwrap();
  match dtype_ident.to_string().as_str() {
    "f64" => Ok(rblock::DType::Real),
    "bool" => Ok(rblock::DType::Bool),
    x => cerror!(dtype_ident, "Unknown field dtype {}", x)
  }
}


// dataflow_core::system::SystemStorage
pub fn generate_new(block_repr: &rblock::Block) -> TokenStream2 {
  let block_ident = syn::Ident::new(
    &block_repr.name, block_repr.span.unwrap()
  );
  let field_expr = block_repr.r_param.iter().map(
    |f| {
      let f_name = syn::Ident::new(&f.field.name, f.field.span.unwrap());
      quote!(#f_name: "4")
    }
  ); 
    // #(#field_expr),*
  // cp: Parameter::<f64>::new(storage, counters.next_r_param()),
  // h: Parameter::<f64>::new(storage, counters.next_r_param()),
  // area: Parameter::<f64>::new(storage, counters.next_r_param()),
  // t_amb: Parameter::<f64>::new(storage, counters.next_r_param()),
  // t: ContinuousState::new(storage, counters.next_r_state()),
  // qdot: Input::new(storage),
  // t_out: Output::new(storage, counters.next_r_out()),

  quote!{
    impl #block_ident {
      pub fn new<'a>(
        storage: &'a dyn dataflow_core::system::SystemStorage, 
        counters: &mut dataflow_core::system::SystemCounters
      ) -> #block_ident<'a> {
        #block_ident {
          #(#field_expr),* ,
        }
      }
    }    
  }
}

fn generate_field_new(field: &rblock::BlockField) -> TokenStream2 {
  let field_ident = syn::Ident::new(&field.name, field.span.unwrap());

  quote!{
    #field_ident: Parameter::<f64>::new(storage, counters.next_r_param())
  }
}

pub fn generate_size(block_repr: &rblock::Block) -> TokenStream2 {
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