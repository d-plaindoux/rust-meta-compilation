use crate::data::Comprehension;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

mod data;
mod parser;
mod tokenizer;

#[proc_macro]
pub fn foreach(input: TokenStream) -> TokenStream {
    let c = parse_macro_input!(input as Comprehension);
    quote! { #c }.into()
}
