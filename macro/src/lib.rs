#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

extern crate proc_macro;

use proc_macro::TokenStream;
use syn::DeriveInput;

#[proc_macro_derive(ID)]
pub fn my_macro_here_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    // Build the output, possibly using quasi-quotation
    let expanded = quote! {
        impl crate::container::arcmap::WithId for #name {
            fn set_id(&mut self, id: u32) {
                self.id = id;
            }

            fn id(&self) -> u32 {
                self.id
            }
        }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}
