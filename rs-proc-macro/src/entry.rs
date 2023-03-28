use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote, quote_spanned, ToTokens};

pub(crate) fn main(item: TokenStream) -> TokenStream {
    let mut input: syn::ItemFn = match syn::parse(item.clone()) {
        Ok(it) => it,
        Err(e) => return token_stream_with_error(item, e),
    };

    // If type mismatch occurs, the current rustc points to the last statement.
    let (_last_stmt_start_span, last_stmt_end_span) = {
        let mut last_stmt = input
            .block
            .stmts
            .last()
            .map(ToTokens::into_token_stream)
            .unwrap_or_default()
            .into_iter();
        // `Span` on stable Rust has a limitation that only points to the first
        // token, not the whole tokens. We can work around this limitation by
        // using the first/last span of the tokens like
        // `syn::Error::new_spanned` does.
        let start = last_stmt.next().map_or_else(Span::call_site, |t| t.span());
        let end = last_stmt.last().map_or(start, |t| t.span());
        (start, end)
    };

    let body = &input.block;
    let brace_token = input.block.brace_token;
    let body_ident = quote! { || #body };

    let block_expr = quote_spanned! {last_stmt_end_span=>
        #[allow(clippy::expect_used, clippy::diverging_sub_expression)]
        {
            return rs_core::runtime::Builder::new()
                .build()
                .expect("Failed building the Runtime")
                .start(#body_ident);
        }
    };

    input.block = syn::parse2(quote! {
        {
            #block_expr
        }
    })
    .expect("Parsing failure");
    input.block.brace_token = brace_token;

    let result = quote! {
        #input
    };

    result.into()
}

fn token_stream_with_error(mut tokens: TokenStream, error: syn::Error) -> TokenStream {
    tokens.extend(TokenStream::from(error.into_compile_error()));
    tokens
}
