use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Data, Fields};


#[proc_macro_derive(SerializeNumberStruct)]
pub fn serialize_number_struct(input : TokenStream) -> TokenStream {
    let ast : DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;

    let serialized_fields = match &ast.data {
        Data::Struct(data_struct) =>{
            match &data_struct.fields {
                Fields::Named(fields) => {

                    let field_serialization = fields.named.iter().map(|field|{
                        let field_name = &field.ident;

                        quote! {
                            result.extend_from_slice(&self.#field_name.to_be_bytes());
                        }
                    });

                    quote! {
                        #(#field_serialization)*;
                    }

                }
                _ => panic!("Only named fields are supported")       
            }
        }
        _ => panic!("Only structs are supported")
    };


    let generated = quote! {
        impl Serialize for #name {
            fn serialize(&self) -> Vec<u8> {
                let mut result = Vec::new();
                #serialized_fields
                result
            }
        }
    };

    generated.into()//converst proc_macro::macro2::TokenStream to proc_macro::macro::TokenStream
}
