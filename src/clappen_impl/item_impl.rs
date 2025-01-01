use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use std::str::FromStr;
use syn::spanned::Spanned;
use syn::{parse_quote, Ident, ItemImpl};

use super::ProcessItem;
use crate::helper;

impl ProcessItem for ItemImpl {
  fn process(
    &mut self,
    default_prefix: String,
    attrs_prefix: String,
    prefixed_fields: Vec<String>,
  ) -> syn::Result<TokenStream> {
    let item = &self;
    let mut self_ty = item.self_ty.to_token_stream();

    let prefix = helper::snake_case(helper::prefix(&[
      default_prefix.as_str(),
      attrs_prefix.as_str(),
    ]));

    // handle impl generics and such
    let generics = self.generics.to_token_stream();
    let before_type = match self.trait_.clone(){
      Some((Some(not), name, for_)) => quote! {#generics #not #name #for_},
      Some((None, name, for_)) => quote! {#name #for_},
      None => todo!()
    };
    
    // Total hack to get this to work with Into and such
    if self.unsafety.is_some() && prefix.is_empty(){
      return Ok(TokenStream::new());
    }
    
    // handle impl ty prefix
    if !prefix.is_empty() {
      let mut ident = item.self_ty.to_token_stream().to_string();
      ident.insert_str(0, &helper::camel_case(prefix.to_owned()));

      self_ty = Ident::new(&ident, ident.span()).to_token_stream();
    }

    // handle renaming of self fields references
    if !prefix.is_empty() {
      for i in self.items.iter_mut() {
        for field in &prefixed_fields {
          let content = i.to_token_stream().to_string();

          let origin = format!("self.{}", field);
          let replace = format!("self.{}_{}", prefix, field);
          let content = content.replace(&origin, &replace);

          let token = TokenStream::from_str(content.as_str())?;
          *i = parse_quote! {#token};
        }
      }
    }

    let doc_prefixed_fields = prefixed_fields.join(",");
    let items = &self.items;

    Ok(quote! {
        #[doc=concat!(concat!(" Fields with prefix: [", #doc_prefixed_fields, "]"))]
        impl #before_type #self_ty {
            #(#items)*
        }
    })
  }
}
