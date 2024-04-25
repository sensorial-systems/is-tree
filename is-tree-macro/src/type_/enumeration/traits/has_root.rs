use proc_macro2::TokenStream;
use quote::quote;

use crate::type_::Enumeration;
use crate::traits::AttributeQuery;

pub fn impl_has_root(enumeration: &Enumeration) -> TokenStream {
    let name = &enumeration.name;
    let generics = &enumeration.generics;
    let self_ = quote! { #name #generics };
    let reference = enumeration
        .named_attribute_value(vec!["tree", "reference"])
        .expect("#[tree(reference = \"type\")] not found in the enumeration.");

    let variants = enumeration.variants.iter().map(|variant| {
        let variant_name = &variant.variant.ident;
        quote! {
            #name::#variant_name(value) => value.root(),
        }
    }).collect::<TokenStream>();

    let variant = enumeration.variants.first().map(|first| &first.variant).expect("Enum must have at least one variant");
    let variant = variant.fields.iter().next().expect("Variant must have at least one field");
    
    quote! {
        // impl #generics ::is_tree::KnowsRoot<'a> for #self_ {
        //     type Root = <#variant as ::is_tree::KnowsRoot<'a>>::Root;
        // }

        // impl<'a> ::is_tree::HasRoot<'a> for &'a #reference {
        //     fn root(self) -> Self::Root {
        //         match self {
        //             #variants
        //         }
        //     }
        // }
    }
}