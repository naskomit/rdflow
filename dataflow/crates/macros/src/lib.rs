use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};
use quote::{quote};
use block::{create_block_repr, generate_size, generate_new};

mod block;
mod types;
mod utils;
mod parsers;

#[proc_macro_derive(Block, attributes(default, initial))]
pub fn derive(input: TokenStream) -> TokenStream {
  let ast = parse_macro_input!(input as DeriveInput);
  let block_repr = create_block_repr(&ast);
  let output = block_repr.and_then(|repr| {
    let size_section = generate_size(&repr);
    let new_section = generate_new(&repr);
    Ok(quote!{
      // #new_section
      #size_section
    })  
  });

  match output {
    Ok(res) => {
      let x: TokenStream = res.into();
      eprintln!("{}", x.to_string());
      x
    },
    Err(e) => e.to_compile_error().into()
  }
}