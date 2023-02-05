use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields};

#[proc_macro_derive(Getters)]
pub fn getters(input: TokenStream1) -> TokenStream1 {
    let input = parse_macro_input!(input as DeriveInput);
    expand_getters(input).into()
}

fn expand_getters(input: DeriveInput) -> TokenStream {
    let fields = match input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => fields.named,
        _ => panic!("this derive macro only works on structs with named fields"),
    };

    let getters = fields.into_iter().map(|f| {
        let field_name = f.ident;
        let field_ty = f.ty;

        quote! {
            pub fn #field_name(&self) -> &#field_ty {
                &self.#field_name
            }
        }
    });

    let st_name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    quote! {
        #[automatically_derived]
        impl #impl_generics #st_name #ty_generics #where_clause {
            #(#getters)*
        }
    }
}
