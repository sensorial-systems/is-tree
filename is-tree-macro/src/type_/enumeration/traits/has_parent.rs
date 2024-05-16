use proc_macro2::TokenStream;
use quote::quote;

use crate::type_::Enumeration;

pub fn impl_has_parent(enumeration: &Enumeration) -> TokenStream {
    let name = &enumeration.name;
    let generics = &enumeration.generics;
    let self_ = quote! { #name #generics };
    let generics = enumeration.generics_with_lifetime();
    let variants = enumeration.variants.iter().map(|variant| {
        let variant_name = &variant.variant.ident;
        quote! {
            #name::#variant_name(value) => value.parent().into(),
        }
    }).collect::<TokenStream>();
    let clauses = enumeration.generics_where_clauses(|type_| quote! { #type_: Clone });

    quote! {
        impl #generics ::is_tree::KnowsParent<'a> for #self_ {
            type Parent = #self_;
        }

        impl #generics ::is_tree::HasParent<'a> for &'a #self_
        where #clauses
        {
            fn parent(self) -> Self::Parent {
                match self {
                    #variants
                }
            }
        }

        // TODO: Implement mutable version
    }
}
