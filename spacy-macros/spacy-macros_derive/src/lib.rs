extern crate proc_macro2;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};

use syn::{parse::Parser, parse_macro_input, DeriveInput, Ident};

#[proc_macro_attribute]
pub fn add_uuid(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    if let syn::Data::Struct(ref mut struct_data) = &mut ast.data {
        if let syn::Fields::Named(fields) = &mut struct_data.fields {
            let new_param_vector: &Vec<TokenStream2> = &fields
                .named
                .iter()
                .map(|f| {
                    let i = f.ident.clone().unwrap();
                    let t = f.ty.clone().to_token_stream();
                    quote! {
                        #i: #t
                    }
                })
                .collect();

            fields.named.push(
                syn::Field::parse_named
                    .parse2(quote! { pub uuid: Option<Uuid> })
                    .unwrap(),
            );

            let all_param_names: &Vec<Ident> = &fields
                .named
                .iter()
                .map(|f| f.ident.clone().unwrap())
                .collect();

            let implementation = quote! {
                impl #name {
                    pub fn new(#(#new_param_vector),*) -> Self {
                        let uuid = Some(Uuid::new_v4());

                        #name {
                            #(#all_param_names),*
                        }
                    }

                    fn hey(){
                      println!("hey")
                    }
                }
            };

            return quote! {
                #ast
                #implementation
            }
            .into();
        }
    }
    panic!("`add_uuid` has to be used with structs ")
}
