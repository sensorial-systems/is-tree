use quote::quote;

pub(crate) fn impl_has_path_segment(ast: &syn::DeriveInput, data: &syn::DataStruct) -> proc_macro2::TokenStream {
    let structure_name = &ast.ident;
    data.fields.iter().find_map(|field| {
        field.attrs.iter().find(|attr| {
            attr.path().is_ident("tree") && attr.parse_args::<syn::Path>().map(|path| path.is_ident("path_segment")).unwrap_or_default()
        }).map(|_| {
            let field_name = &field.ident;
            quote! {
                impl ::is_tree::HasPathSegment for #structure_name {
                    fn path_segment(&self) -> &String {
                        &self.#field_name
                    }
                }
            }
        })
    }).unwrap_or_else(|| {
        quote! {}
    })
}