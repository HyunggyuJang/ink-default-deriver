extern crate proc_macro;
extern crate quote;
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(DataDefault)]
pub fn derive(input: TokenStream) -> TokenStream {
    let structure = syn::parse::<syn::ItemStruct>(input).unwrap();
    let ident = structure.ident;
    let fields = structure.fields;
    let fields_defaults = fields.iter().map(|field| {
        let field_ident = &field.ident;
        let field_ty = &field.ty;
        let default_impl =
            if is_ascribed_accountid_ty(field_ty) {
                quote!(openbrush::traits::ZERO_ADDRESS.into())
            } else {
                quote!(Default::default())
            };
        quote! {
            #field_ident: #default_impl,
        }
    });
    quote! {
        impl Default for #ident {
            fn default() -> Self {
                Self {
                    #(#fields_defaults)*
                }
            }
        }
    }.into()
}

fn is_accountid_ty(ty: &syn::Type) -> bool {
    if let syn::Type::Path(syn::TypePath { path: syn::Path { segments, .. }, .. }, ..) = ty {
        if let Some(syn::PathSegment { ident, .. }) = segments.last() {
            if ident == "AccountId" {
                return true
            }
        }
    };
    false
}

fn is_ascribed_accountid_ty(ty: &syn::Type) -> bool {
    if let syn::Type::Path(syn::TypePath { qself: Some(qself), .. }, ..) = ty {
        return is_accountid_ty(qself.ty.as_ref())
    };
    false
}

#[test]
fn test_quote() {
    assert_eq!(quote! { hi : test }.to_string(), "hi: test")
}
