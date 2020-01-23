extern crate proc_macro;

mod errors;
mod funcs;
mod names;
mod parse;
mod types;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

use errors::define_error_trait;
use funcs::define_func;
use names::Names;
use types::define_datatype;

#[proc_macro]
pub fn from_witx(args: TokenStream) -> TokenStream {
    let args = TokenStream2::from(args);
    let witx_paths = parse::witx_paths(args).expect("parsing macro arguments");

    let names = Names::new(); // TODO parse the names from the invocation of the macro, or from a file?

    let doc = witx::load(&witx_paths).expect("loading witx");

    let types = doc.typenames().map(|t| define_datatype(&names, &t));

    let modules = doc.modules().map(|module| {
        let modname = names.module(&module.name);
        let fs = module.funcs().map(|f| define_func(&names, &f));
        quote!(
            mod #modname {
                use super::*;
                use super::types::*;
                use memory::*;
                #(#fs)*
            }
        )
    });

    let error_trait = define_error_trait(&names, &doc);

    TokenStream::from(quote!(
        mod types {
            #(#types)*
            #error_trait
        }
        #(#modules)*
    ))
}
