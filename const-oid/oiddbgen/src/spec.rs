use proc_macro2::{Ident, TokenStream};
use quote::quote;

use crate::node::Node;

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Spec(Vec<Node>);

impl Spec {
    pub fn push(&mut self, value: Node) {
        self.0.push(value)
    }

    pub fn symbols(&self, path: TokenStream) -> TokenStream {
        let mut stream = TokenStream::default();

        for n in &self.0 {
            let symb = n.symbol();
            stream.extend(quote! { #path::#symb, })
        }

        stream
    }

    pub fn module(&self, spec: &Ident) -> TokenStream {
        let mut defs = TokenStream::default();

        for n in &self.0 {
            defs.extend(n.definition())
        }

        quote! {
            pub mod #spec {
                #defs
            }
        }
    }
}