use proc_macro2::TokenStream;
use quote::quote;

use crate::type_::Enumeration;

pub fn impl_branches(enumeration: &Enumeration) -> TokenStream {
    let name = &enumeration.name;
    let generics = &enumeration.generics;
    let self_ = quote! { #name #generics };
    let generics = enumeration.generics_with_lifetime();

    let generics_clauses = enumeration.generics_where_clauses(|type_param| {
        quote! {
            #type_param: Clone + ::is_tree::HasBranches<'a> + 'a,
            <#type_param as ::is_tree::KnowsBranches<'a>>::Branches: ::is_tree::KnowsVisitor<'a>,
            <<#type_param as ::is_tree::KnowsBranches<'a>>::Branches as ::is_tree::KnowsVisitor<'a>>::Visitor: Into<Self::Branches>
        }
    });
    let fields_clauses = enumeration.variants.iter().map(|variant| {
        let field = variant.fields().first().unwrap().clone();
        let field = match field.identifier().to_string().as_str() {
            "Box" => field.inner_types().first().unwrap().clone(),
            _ => field
        };
        let visitor = field.path;
        quote! {
            <#visitor as ::is_tree::KnowsBranches<'a>>::Branches: ::is_tree::HasVisitorConstructor<'a, Value = <<#visitor as ::is_tree::KnowsValue<'a>>::Value as ::is_tree::KnowsBranches<'a>>::Branches>,
            #visitor: Into<<<#visitor as ::is_tree::KnowsBranches<'a>>::Branches as ::is_tree::KnowsParent<'a>>::Parent>,
        }
    }).collect::<TokenStream>();
    let clauses = quote! {
        #self_: ::is_tree::KnowsParent<'a, Parent = #self_>,
        #generics_clauses
        #fields_clauses
    };

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

        impl #generics ::is_tree::HasBranches<'a> for &'a #self_
        where #clauses
        {
            fn branches(self) -> impl Iterator<Item = Self::Branches> {
                fn longer_ref<'longer, T>(t: &T) -> &T { t }
                match self {
                    #variants
                }
            }
        }

        impl #generics ::is_tree::HasBranches<'a> for #self_
        where #clauses
        {
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