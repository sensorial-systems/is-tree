use proc_macro2::TokenStream;
use quote::quote;

use crate::{traits::AttributeQuery, type_::Enumeration};

pub fn impl_knows_relative_access(enumeration: &Enumeration) -> TokenStream {
    let structure_name = &enumeration.name;
    let visitor = enumeration
        .named_attribute_value(vec!["tree", "visitor"])
        .unwrap_or_else(|| structure_name.clone().into());
    let reference = enumeration
        .named_attribute_value(vec!["tree", "reference"])
        .expect("#[tree(reference = \"type\")] not found in the enumeration.");
    quote! {
        impl<'a> ::is_tree::KnowsRelativeAccessType<'a> for #reference  {
            type RelativeType = #visitor;
        }
    }
}

pub fn impl_has_relative_access(enumeration: &Enumeration) -> TokenStream {
    let name = &enumeration.name;
    let generics = &enumeration.generics;
    let _self = quote! { #name #generics };
    let reference = enumeration
        .named_attribute_value(vec!["tree", "reference"])
        .expect("#[tree(reference = \"type\")] not found in the enumeration.");

    let variants = enumeration.variants.iter().map(|variant| {
        let variant_name = &variant.variant.ident;
        quote! {
            #name::#variant_name(value) => longer_ref(value).relative(path).map(|value| value.into()),
        }
    }).collect::<TokenStream>();

    quote! {
        impl<'a> ::is_tree::HasRelativeAccess<'a> for #reference {
            fn relative<K>(self, path: impl IntoIterator<Item = K>) -> Option<Self::RelativeType>
            where K: Into<String>
            {
                #[inline]
                fn longer_ref<'longer, T>(t: &T) -> &'longer T { unsafe { &*(t as *const T) } }
                match &self {
                    #variants
                }
            }
        }
    }
}

pub fn impl_relative_access(enumeration: &Enumeration) -> TokenStream {
    let knows_relative_access = impl_knows_relative_access(enumeration);
    let has_relative_access = impl_has_relative_access(enumeration);
    quote! {
        #knows_relative_access
        #has_relative_access
    }
}