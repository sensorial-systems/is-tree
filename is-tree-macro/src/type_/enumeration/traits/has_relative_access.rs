use proc_macro2::TokenStream;
use quote::quote;

use crate::type_::Enumeration;

pub fn impl_relative_access(enumeration: &Enumeration) -> TokenStream {
    let name = &enumeration.name;
    let generics = &enumeration.generics;
    let self_ = quote! { #name #generics };
    let generics = enumeration.generics_with_lifetime();
    let variants = enumeration.variants.iter().map(|variant| {
        let variant_name = &variant.variant.ident;
        quote! {
            #name::#variant_name(value) => unsafe { ::is_tree::unsafe_::longer_ref(value).relative(path).map(|value| value.into()) },
        }
    }).collect::<TokenStream>();
    let clauses = enumeration.variants.iter().map(|variant| {
        let field = variant.fields().first().unwrap().clone();
        let field = match field.identifier().to_string().as_str() {
            "Box" => field.inner_types().first().unwrap().clone(),
            _ => field
        };
        let visitor = field.path;
        quote! {
            #visitor: Into<Self::RelativeAccess> + Clone + ::is_tree::KnowsRelativeAccess<'a, RelativeAccess = Self> + 'a,
             &'a #visitor: ::is_tree::HasValue<'a> + ::is_tree::HasParent<'a> + ::is_tree::HasRoot<'a> + ::is_tree::HasGet<'a>,
            <&'a #visitor as ::is_tree::KnowsParent<'a>>::Parent: Into<Self>,
            <&'a #visitor as ::is_tree::KnowsRoot<'a>>::Root: Into<Self>,
            <&'a #visitor as ::is_tree::KnowsRelativeAccess<'a>>::RelativeAccess: Into<Self>,
            <&'a #visitor as ::is_tree::KnowsBranches<'a>>::Branches: Into<Self> + ::is_tree::HasPathSegment,
        }
    }).collect::<TokenStream>();

    quote! {
        impl #generics ::is_tree::KnowsRelativeAccess<'a> for #self_  {
            type RelativeAccess = #self_;
        }
        
        impl #generics ::is_tree::HasRelativeAccess<'a> for #self_
        where #clauses
        {
            fn relative<K>(self, path: impl IntoIterator<Item = K>) -> Option<Self::RelativeAccess>
            where K: Into<String>
            {
                match &self {
                    #variants
                }
            }
        }            
    }
}
