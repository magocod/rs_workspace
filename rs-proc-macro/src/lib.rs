#![allow(clippy::needless_doctest_main)]

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::Parser;
use syn::{parse, parse_macro_input, DeriveInput};

mod derives;
mod entry;

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

/// ## Usage
///
/// ### Using default runtime
///
/// ```rust
/// #[rs_proc_macro::main]
/// fn main() {
///     println!("Hello world");
/// }
/// ```
///
/// Equivalent code not using `#[rs_proc_macro::main]`
///
/// ```rust
/// fn main() {
///     rs_core::runtime::Builder::new()
///         .build()
///         .expect("Failed building the Runtime")
///         .start(|| println!("Hello world"));
/// }
/// ```
#[proc_macro_attribute]
pub fn main(_args: TokenStream, item: TokenStream) -> TokenStream {
    entry::main(item)
}

// /// ## Usage
// ///
// /// ```rust
// /// #[rs_proc_macro::main]
// /// fn main() {
// ///     println!("Hello world");
// /// }
// /// ```
// ///
// /// Equivalent code not using `#[rs_proc_macro::main]`
// ///
// /// ```rust
// /// fn main() {
// ///     rs_core::runtime::Builder::new()
// ///         .build()
// ///         .expect("Failed building the Runtime")
// ///         .start(|| println!("Hello world"));
// /// }
// /// ```
// #[proc_macro_attribute]
// pub fn main(_args: TokenStream, item: TokenStream) -> TokenStream {
//     let mut input: syn::ItemFn = match syn::parse(item.clone()) {
//         Ok(it) => it,
//         Err(e) => return token_stream_with_error(item, e),
//     };
//
//     // If type mismatch occurs, the current rustc points to the last statement.
//     let (_last_stmt_start_span, last_stmt_end_span) = {
//         let mut last_stmt = input
//             .block
//             .stmts
//             .last()
//             .map(ToTokens::into_token_stream)
//             .unwrap_or_default()
//             .into_iter();
//         // `Span` on stable Rust has a limitation that only points to the first
//         // token, not the whole tokens. We can work around this limitation by
//         // using the first/last span of the tokens like
//         // `syn::Error::new_spanned` does.
//         let start = last_stmt.next().map_or_else(Span::call_site, |t| t.span());
//         let end = last_stmt.last().map_or(start, |t| t.span());
//         (start, end)
//     };
//
//     let body = &input.block;
//     let brace_token = input.block.brace_token;
//     let body_ident = quote! { || #body };
//
//     let block_expr = quote_spanned! {last_stmt_end_span=>
//         #[allow(clippy::expect_used, clippy::diverging_sub_expression)]
//         {
//             return rs_core::runtime::Builder::new()
//                 .build()
//                 .expect("Failed building the Runtime")
//                 .start(#body_ident);
//         }
//     };
//
//     input.block = syn::parse2(quote! {
//         {
//             #block_expr
//         }
//     })
//     .expect("Parsing failure");
//     input.block.brace_token = brace_token;
//
//     let result = quote! {
//         #input
//     };
//
//     result.into()
// }

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

/// ### Usage
///
/// ```
/// use rs_proc_macro::Hello;
///
/// #[derive(Hello)]
/// pub struct Example {}
///
/// let e = Example {};
/// e.hello();
/// ```
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
