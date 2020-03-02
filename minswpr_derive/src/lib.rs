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

#[proc_macro_derive(Layout)]
pub fn layout_macro_derive(input: TokenStream) -> TokenStream {
    impl_layout(&syn::parse(input).unwrap())
}

fn impl_layout(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let generics = &ast.generics;
    let gen = quote! {
        impl #generics crate::layout::Layout #generics for #name #generics {
            fn components(&self) -> &crate::layout::ComponentMap {
                &self.components
            }

            fn components_mut(&mut self) -> &mut crate::layout::ComponentMap #generics {
                &mut self.components
            }
        }
    };
    gen.into()
}
