extern crate proc_macro;
use std::{fs::File, io::{Write}, path::Path};
use std::env;

use proc_macro::TokenStream;
use syn::{AttributeArgs, ItemEnum, Lit, parse_macro_input};
use quote::{ToTokens, quote};

#[proc_macro_attribute]
pub fn manifest(attr: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AttributeArgs);
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let cargo_root = Path::new(manifest_dir.as_str());
    let lit = args
        .into_iter()
        .find_map(|meta|{
            match meta {
                syn::NestedMeta::Lit(Lit::Str(lit)) => Some(lit),
                _ => None
            }
        }).unwrap().value();
    
    let target_path = cargo_root.join(Path::new("pkg/unko.txt"));
    println!("hoge: {}",target_path.as_path().to_str().unwrap());
    // output(target_path.as_path(), lit.as_str()).unwrap();
    let item = parse_macro_input!(input as ItemEnum);
    let enum_name = item.ident.clone();
    let match_expr  = item.variants.iter().map(|v| {
        &v.ident
    }).map(|var_name|{
        quote! {
            #enum_name::#var_name => { stringify!(#var_name) },
        }
    }).fold(quote!(), |mut acc,cur| {
        acc.extend(cur);
        acc
    });
    let gen = quote! {
        impl ToString for #enum_name {
            fn to_string(&self) -> String {
                let name = match self {
                    #match_expr
                };
                format!(
                    "cls--{}--{}--{}",
                    module_path!().to_string(),
                    stringify!(#enum_name),
                    name
                )
            }
        }
        impl Into<yew::Classes> for #enum_name {
            fn into(self) -> yew::Classes {
                yew::classes!(self.to_string())
            }
        }
    };
    let mut input = item.into_token_stream();
    input.extend(gen);
    input.into()
}

fn output(path: &Path,content: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(path)?;
    write!(file, "{}", content)?;
    file.flush()?;
    Ok(())
    
}