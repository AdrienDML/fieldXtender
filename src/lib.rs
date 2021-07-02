use proc_macro::TokenStream;
mod add_field;
use add_field::*;
use syn::{
    parse_macro_input,
};
use quote::quote;

#[proc_macro_attribute]
pub fn add_field(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(input as syn::ItemStruct);
    let arg = parse_macro_input!(args as Args);

    item_struct.add_field(arg.field);

    return quote! {
        #item_struct
    }
    .into();
}