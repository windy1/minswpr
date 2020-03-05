extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn::{self, DeriveInput};

#[proc_macro_derive(AsAny)]
pub fn as_any_macro_derive(input: TokenStream) -> TokenStream {
    impl_as_any(&syn::parse(input).unwrap())
}

fn impl_as_any(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let generics = &ast.generics;
    let gen = quote! {
        impl #generics AsRef<dyn ::std::any::Any> for #name #generics {
            fn as_ref(&self) -> &dyn ::std::any::Any {
                self
            }
        }
    };
    gen.into()
}
