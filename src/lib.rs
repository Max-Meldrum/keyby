extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields};

#[proc_macro_attribute]
pub fn key_by(key: TokenStream, input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as DeriveInput);
    let name = &item.ident;

    let _fields = match &item.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => &fields.named,
        _ => panic!("expected a struct with named fields"),
    };

    let keys: Vec<proc_macro2::TokenStream> = key
        .to_string()
        .trim()
        .split(",")
        .map(|k| {
            let struct_field = Ident::new(&k.trim(), Span::call_site());
            quote! { self.#struct_field.hash(state); }
        })
        .collect();

    let output: proc_macro2::TokenStream = {
        quote! {
            #item
            impl ::std::hash::Hash for #name {
                fn hash<H: ::std::hash::Hasher>(&self, state: &mut H) {
                    #(#keys)*
                }
            }
        }
    };

    proc_macro::TokenStream::from(output)
}
