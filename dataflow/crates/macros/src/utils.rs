macro_rules! cerror {
  ($elem: ident, $msg: literal) => {
    return Err(syn::Error::new($elem.span(), $msg))
  };

  ($elem: ident, $msg: literal, $($args: expr),+) => {
    return Err(syn::Error::new($elem.span(), format!($msg, $($args),+)))
  };
}

macro_rules! lerror {
  ($elem: ident, $msg: literal) => {
    Err(syn::Error::new($elem.span(), $msg))
  };

  ($elem: ident, $msg: literal, $($args: expr),+) => {
    Err(syn::Error::new($elem.span(), format!($msg, $($args),+)))
  };
}

pub(crate) use cerror;
pub(crate) use lerror;

