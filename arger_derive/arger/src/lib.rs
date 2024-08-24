
mod tests;



use proc_macro::{Span, TokenStream};
use quote::quote;
use syn::{self, meta, parse::Parse, parse_macro_input, Attribute, Data, DeriveInput, ItemFn, Meta, PathSegment};


// ! this is the only export thing that we can make when dealing with the procedural macros
#[proc_macro_derive(Parser , attributes(arg))]
pub fn parse(input: TokenStream) -> TokenStream {

  let ast = syn::parse(input);

  if let Ok(ast) = ast {

    let tokens = extract_attr_code(&ast);

    for token in tokens {
      println!("the token is {}", token);
    };

    impl_args(&ast)

  } else {
    panic!("the stream of token is not valid")
  }
}

fn extract_attr_code(ast : &DeriveInput) -> Vec<String> {
  let mut tokens = Vec::new();
  for attr in &ast.attrs {
    match &attr.meta {
          Meta::List(list) => {
              if list.path.segments[0].ident.to_string() == "arg" {
                  tokens.push(list.path.segments[0].ident.to_string())
              }
          }
          _ => {
            panic!("can't be used that way")
          }
    }
  }

  tokens
}

fn impl_args(ast : &DeriveInput) -> TokenStream {
  let name = &ast.ident;
  // * we need a parser that can generate a code for the underline structure
  let gen = quote! {
      impl Parser for #name {
          fn parse() {
              println!("parser implemented on {}!", stringify!(#name));
          }
      }
  };
  gen.into()
}