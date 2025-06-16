// expr ::=
//   ...
//   | "for" comprehension
//
// comprehension ::=
//   ident "<-" expr ("if" expr)? ("yield" expr | comprehension)

use crate::data::ForEach;
use syn::parse::ParseStream;

impl syn::parse::Parse for ForEach {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident = input.parse::<syn::Ident>()?;
        let _ = input.parse::<syn::Token![<-]>()?;
        let value = input.parse::<syn::Expr>()?;

        let condition = if input.lookahead1().peek(syn::Token![if]) {
            let _ = input.parse::<syn::Token![if]>()?;
            Some(input.parse::<syn::Expr>()?)
        } else {
            None
        };

        if input.lookahead1().peek(syn::Token![yield]) {
            let _ = input.parse::<syn::Token![yield]>()?;
            let result = input.parse::<syn::Expr>()?;

            Ok(ForEach::MappingAndYield {
                ident,
                value,
                condition,
                result,
            })
        } else {
            let next = Box::new(input.parse()?);

            Ok(ForEach::ChainedMapping {
                ident,
                value,
                condition,
                next,
            })
        }
    }
}
