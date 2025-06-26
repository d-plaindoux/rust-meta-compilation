use crate::data::Comprehension;
use proc_macro2::TokenStream;
use quote::quote;

impl quote::ToTokens for Comprehension {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.to_token_stream())
    }

    fn to_token_stream(&self) -> TokenStream {
        match self {
            Comprehension::MappingAndYield {
                ident,
                value,
                condition,
                result,
            } => {
                if condition.is_none() {
                    quote!(#value.map(move |#ident| #result))
                } else {
                    let cond = condition.as_ref().unwrap();
                    quote!(#value.filter(move |&#ident| #cond).map(move |#ident| #result))
                }
            }
            Comprehension::ChainedMapping {
                ident,
                value,
                condition,
                comprehension: next,
            } => {
                let next = next.to_token_stream();

                if condition.is_none() {
                    quote!(#value.map(move |#ident| #next).flatten())
                } else {
                    let cond = condition.as_ref().unwrap();
                    quote!(#value.filter(move |&#ident| #cond).map(move |#ident| #next).flatten())
                }
            }
        }
    }
}
