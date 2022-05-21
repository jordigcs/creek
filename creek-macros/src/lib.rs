extern crate proc_macro;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, Data, Fields};

#[proc_macro_derive(Event, attributes(EventContent))]
pub fn derive_event(_item: TokenStream) -> TokenStream {
    TokenStream::new()
}
