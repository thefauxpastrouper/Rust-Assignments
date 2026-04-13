use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Lit, Meta};

#[proc_macro_derive(HelloWorld, attributes(HelloWorldName))]
pub fn my_trait_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident; // The Struct identifier (e.g., Rustacean)

    // 1. Extract the value from #[HelloWorldName = "the big Pancakes"]
    let custom_name = ast.attrs.iter().find_map(|attr| {
        if attr.path().is_ident("HelloWorldName") {
            // In syn 2.0, we check if it's a NameValue pair (key = value)
            if let Meta::NameValue(nv) = &attr.meta {
                // Check if the value is a string literal
                if let syn::Expr::Lit(expr_lit) = &nv.value {
                    if let Lit::Str(lit_str) = &expr_lit.lit {
                        return Some(lit_str.value());
                    }
                }
            }
        }
        None
    });

    // 2. Fallback to the struct name if no attribute is found
    let output_name = custom_name.unwrap_or_else(|| name.to_string());

    // 3. Generate the implementation
    let expanded = quote! {
        // We MUST implement for #name (the type), not the string literal
        impl HelloWorld for #name {
            fn hello() {
                // We use the string we extracted (#output_name) in the println!
                println!("Hello World, My name is {}", #output_name);
            }
        }
    };

    TokenStream::from(expanded)
}