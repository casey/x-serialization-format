macro_rules! assert_derive_x_expansion_eq {
  {
    $item:item,
    $($expansion:tt)*
  } => {
    {
      let expansion =
crate::derive_x_inner(quote::quote!($item)).expect("proc macro invocation
failed");

      let have = expansion.to_string();

      if let Err(err) = syn::parse2::<File>(expansion.clone()) {
        panic!("Expansion parsing failed: {}\n{}", err, have);
      }

      let want = quote::quote!($($expansion)*).to_string();
      pretty_assertions::assert_eq!(have, want);
    }
  }
}

macro_rules! assert_derive_x_error_match {
  {
    $item:item,
    $want:pat
  } => {
    {
      let have = crate::derive_x_inner(quote::quote!($item));
      match have {
        Ok(_) => panic!("Derive unexpectedly succeeded!"),
        Err($want) => {}
        Err(err) => panic!("Unexpected error: {:?}", err),
      }
    }
  }
}
