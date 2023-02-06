use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn impl_process(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl #name {
            pub fn step_b(&self) -> u8 {
                let r: u8 = self.step_a();
                r + 1
            }
        }
    };
    gen.into()
}
