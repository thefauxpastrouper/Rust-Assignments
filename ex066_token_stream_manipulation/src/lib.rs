use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemFn};
use quote::quote;

#[proc_macro_attribute]
pub fn trace(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input as ItemFn);
    let fn_name = &input_fn.sig.ident;
    let fn_sig = &input_fn.sig;
    let fn_body = &input_fn.block;

    let expanded = quote! {
        #fn_sig {
            println!("Entering function: {}", stringify!(#fn_name));
            #fn_body
        }
    };

    TokenStream::from(expanded)
}
