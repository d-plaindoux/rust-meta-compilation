use crate::data::ForEach;
use proc_macro2::TokenStream;
use quote::quote;

impl quote::ToTokens for ForEach {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.to_token_stream())
    }

    fn to_token_stream(&self) -> TokenStream {
        match self {
            ForEach::MappingAndYield {
                ident,
                value,
                condition,
                result,
            } => {
                if condition.is_some() {
                    let cond = condition.as_ref().unwrap();
                    quote!(#value.filter(move |&#ident| #cond).map(move |#ident| #result))
                } else {
                    quote!(#value.map(move |#ident| #result))
                }
            }
            ForEach::ChainedMapping {
                ident,
                value,
                condition,
                next,
            } => {
                let next = next.to_token_stream();

                if condition.is_some() {
                    let cond = condition.as_ref().unwrap();
                    quote!(#value.filter(move |&#ident| #cond).map(move |#ident| #next).flatten())
                } else {
                    quote!(#value.map(move |#ident| #next).flatten())
                }
            }
        }
    }
}
