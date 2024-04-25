use proc_macro2::TokenStream;
use quote::quote;

use crate::type_::Enumeration;

pub fn impl_branches(enumeration: &Enumeration) -> TokenStream {
    let name = &enumeration.name;
    let generics = enumeration.generics_with(quote! { ::is_tree::KnowsBranches<'a> });
    let self_ = quote! { #name #generics };

    let variants = enumeration.variants.iter().map(|variant| {
        let variant_name = &variant.variant.ident;
        quote! {
            #name::#variant_name(value) => longer_ref(value).branches().map(|value| value.into()).collect::<Vec<_>>().into_iter(), // TODO: This needs optimization.
        }
    }).collect::<TokenStream>();

    quote! {
        impl #generics ::is_tree::KnowsBranches<'a> for #self_ {
            type Branches = #self_;
        }

        impl #generics ::is_tree::KnowsBranches<'a> for &'a #self_ {
            type Branches = #self_;
        }

        impl #generics ::is_tree::HasBranches<'a> for &'a #self_ {
            fn branches(self) -> impl Iterator<Item = Self::Branches> {
                fn longer_ref<'longer, T>(t: &T) -> &T { t }
                match self {
                    #variants
                }
            }
        }

        impl #generics ::is_tree::HasBranches<'a> for #self_ {
            fn branches(self) -> impl Iterator<Item = Self::Branches> {
                #[inline]
                fn longer_ref<'longer, T>(t: &T) -> &'longer T { unsafe { &*(t as *const T) } }
                match &self {
                    #variants
                }
            }
        }
    }
}