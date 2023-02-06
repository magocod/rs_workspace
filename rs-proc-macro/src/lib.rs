use proc_macro::TokenStream;
use quote::quote;
use syn::parse::Parser;
use syn::{parse, parse_macro_input, DeriveInput};

mod derives;

#[proc_macro]
pub fn make_answer(_item: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 42 }".parse().unwrap()
}

#[proc_macro_attribute]
pub fn show_streams(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("attr: \"{attr}\"");
    println!("item: \"{item}\"");
    item
}

#[proc_macro_attribute]
pub fn add_field(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(input as DeriveInput);
    match &mut ast.data {
        syn::Data::Struct(ref mut struct_data) => {
            // match &mut struct_data.fields {
            //     syn::Fields::Named(fields) => {
            //         fields.named.push(
            //             syn::Field::parse_named
            //                 .parse2(quote! { pub a: String })
            //                 .unwrap(),
            //         );
            //     }
            //     _ => (),
            // }
            if let syn::Fields::Named(fields) = &mut struct_data.fields {
                fields.named.push(
                    syn::Field::parse_named
                        .parse2(quote! { pub a: String })
                        .unwrap(),
                );
            }

            let q = quote! {
                #ast
            };
            q.into()
        }
        _ => panic!("`add_field` has to be used with structs "),
    }
}

#[proc_macro_derive(Hello)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = parse(input).unwrap();

    // Build the trait implementation
    derives::impl_hello(&ast)
}

#[proc_macro_derive(Process)]
pub fn process_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = parse(input).unwrap();

    // Build the trait implementation
    derives::impl_process(&ast)
}

#[proc_macro_derive(Counter)]
pub fn counter_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = parse(input).unwrap();

    // Build the trait implementation
    derives::impl_counter(&ast)
}
