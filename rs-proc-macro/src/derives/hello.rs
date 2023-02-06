use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn impl_hello(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl #name {
            fn hello(&self) -> String {
                format!("Hello, My name is {}!", stringify!(#name))
            }
        }
    };
    gen.into()
}
