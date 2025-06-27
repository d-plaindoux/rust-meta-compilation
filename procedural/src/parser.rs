/// expr ::=
///   ...
///   | "for" comprehension
///
/// comprehension ::=
///   ident "<-" expr ("if" expr)? ("yield" expr | comprehension)
///   ^____( 1 )____^ ^__( 2 )___^ ^___________( 3 )____________^

use crate::data::Comprehension;
use syn::parse::ParseStream;

impl syn::parse::Parse for Comprehension {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // 1 - ident "<-" expr
        let ident = input.parse::<syn::Ident>()?;
        let _ = input.parse::<syn::Token![<-]>()?;
        let value = input.parse::<syn::Expr>()?;

        // 2 - ("if" expr)?
        let condition = if input.lookahead1().peek(syn::Token![if]) {
            let _ = input.parse::<syn::Token![if]>()?;
            Some(input.parse::<syn::Expr>()?)
        } else {
            None
        };

        // 3 - ("yield" expr | comprehension)
        if input.lookahead1().peek(syn::Token![yield]) {
            let _ = input.parse::<syn::Token![yield]>()?;
            let result = input.parse::<syn::Expr>()?;

            Ok(Comprehension::MappingAndYield {
                ident,
                value,
                condition,
                result,
            })
        } else {
            let next = Box::new(input.parse()?);

            Ok(Comprehension::ChainedMapping {
                ident,
                value,
                condition,
                comprehension: next,
            })
        }
    }
}
