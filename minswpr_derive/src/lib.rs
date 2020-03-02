extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn::{self, DeriveInput};

#[proc_macro_derive(Input)]
pub fn input_macro_derive(input: TokenStream) -> TokenStream {
    impl_input(&syn::parse(input).unwrap())
}

fn impl_input(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let generics = &ast.generics;
    let gen = quote! {
        impl #generics crate::input::Input for #name #generics {
            fn context(&self) -> &crate::Context {
                &self.context
            }
        }
    };
    gen.into()
}
