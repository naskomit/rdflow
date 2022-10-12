use super::types::{Res, GenericType, SimpleType};
use super::utils::{cerror, lerror};
use syn::spanned::Spanned;

pub fn parse_simple_path<'a, T>(segments: T) -> Res<SimpleType> 
  where T: Iterator<Item = &'a syn::PathSegment> {
  let path = segments.map(|segment| {
    if segment.arguments.is_empty() {
      Ok(segment.ident.to_owned())
    } else {
      lerror!(segment, "expected qualified simple type, found {:#?}", segment)
    }
  }).collect::<Res<Vec<syn::Ident>>>()?;
  Ok(SimpleType {qualified_path: path})
}

pub fn parse_qualified_simple_type(tpe: &syn::Type) -> Res<SimpleType> {
  match tpe {
    syn::Type::Path(syn::TypePath {
      path: syn::Path {ref segments, ..}, ..
    }) => parse_simple_path(segments.iter()),
    _ => cerror!(tpe, "expected qualified simple type, found {:#?}", tpe)
  }
}

pub fn parse_qualified_generic_type(tpe: &syn::Type) -> Res<GenericType> {
  let mut tpe_repr = GenericType::default();

  match tpe {
    syn::Type::Path(syn::TypePath {
      path: syn::Path {ref segments, ..}, ..
    }) => {
      let mut base_type = parse_simple_path(
        segments.iter().take(segments.len() - 1)
      )?;

      let last_segment = segments.iter().last().unwrap();
      base_type.qualified_path.push(last_segment.ident.to_owned());
      tpe_repr.base_type = base_type;
      match last_segment.arguments {
        syn::PathArguments::AngleBracketed(
          syn::AngleBracketedGenericArguments {ref args, ..}
        ) => {
          let res: Res<()> = args.iter().map(|arg| { match arg {
            &syn::GenericArgument::Lifetime(syn::Lifetime {ref ident, ..}) => {
              tpe_repr.lifetime = Some(ident.to_owned());
              Ok(())
            },
            &syn::GenericArgument::Type(ref tpe) => {
              let tpe_param = parse_qualified_simple_type(tpe)?;
              tpe_repr.type_params.push(tpe_param);
              Ok(())
            },
            
            _ => cerror!(last_segment, "expected only qualified simple type paths and lifetime arguments"),
          }}).collect();
          res?
      },
        _ => cerror!(tpe, "cannot parse type argumens"),
      }

    },
    _ => cerror!(tpe, "expected qualified generic type, found {:#?}", tpe)
  };


  Ok(tpe_repr)

}