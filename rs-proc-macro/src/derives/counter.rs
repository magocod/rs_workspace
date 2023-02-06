use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn impl_counter(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl #name {
            pub fn new() -> #name {
                #name {
                    count: 0
                }
            }

            pub fn increment(&mut self) {
                self.count = self.count + 1;
            }

            pub fn decrement(&mut self) {
                self.count = self.count - 1;
            }

            pub fn state(&self) -> i32 {
                self.count
            }
        }

        impl std::default::Default for #name {
            fn default() -> #name {
                #name::new()
            }
        }
    };
    gen.into()
}
