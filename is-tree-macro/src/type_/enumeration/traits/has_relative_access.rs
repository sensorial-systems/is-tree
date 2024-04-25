use proc_macro2::TokenStream;
use quote::quote;

use crate::type_::Enumeration;

pub fn impl_relative_access(enumeration: &Enumeration) -> TokenStream {
    let name = &enumeration.name;
    let generics = &enumeration.generics;
    let self_ = quote! { #name #generics };
    let variants = enumeration.variants.iter().map(|variant| {
        let variant_name = &variant.variant.ident;
        quote! {
            #name::#variant_name(value) => longer_ref(value).relative(path).map(|value| value.into()),
        }
    }).collect::<TokenStream>();

    quote! {
        impl #generics ::is_tree::KnowsRelativeAccessType<'a> for #self_ {
            type RelativeType = #self_;
        }

        impl #generics ::is_tree::HasRelativeAccess<'a> for #self_ {
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
