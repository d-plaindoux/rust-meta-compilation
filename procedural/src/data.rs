pub enum Comprehension {
    MappingAndYield {
        ident: syn::Ident,
        value: syn::Expr,
        condition: Option<syn::Expr>,
        result: syn::Expr,
    },
    ChainedMapping {
        ident: syn::Ident,
        value: syn::Expr,
        condition: Option<syn::Expr>,
        comprehension: Box<Comprehension>,
    },
}
