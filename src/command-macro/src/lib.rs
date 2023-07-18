#![allow(clippy::expect_used, clippy::missing_panics_doc, clippy::unwrap_used)]

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, ImplItem, ItemFn, ItemImpl, Lit, Meta, NestedMeta};

#[proc_macro_attribute]
pub fn command(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let function = parse_macro_input!(input as ItemFn);

    let mut target_struct = None;
    let mut ctx_type = quote! { Ctx };
    let mut name = None;
    let mut description = None;
    let mut alias = vec![];

    for arg in args {
        match arg {
            NestedMeta::Meta(Meta::Path(path)) => {
                if target_struct.is_none() {
                    target_struct = path.get_ident().cloned();
                } else {
                    ctx_type = quote! { #path };
                }
            }
            NestedMeta::Meta(Meta::NameValue(nv)) => {
                match nv.path.get_ident().map(ToString::to_string).as_deref() {
                    Some("name") => {
                        if let Lit::Str(lit) = nv.lit {
                            name = Some(lit);
                        }
                    }
                    Some("description") => {
                        if let Lit::Str(lit) = nv.lit {
                            description = Some(lit);
                        }
                    }
                    Some("alias") => {
                        if let Lit::Str(lit) = nv.lit {
                            alias = lit
                                .value()
                                .split(',')
                                .map(|s| s.trim().to_string())
                                .collect();
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    let target_struct = target_struct
        .expect("The 'target_struct' argument is required for the 'command' attribute");
    let name = name.expect("The 'name' argument is required for the 'command' attribute");
    let description =
        description.expect("The 'description' argument is required for the 'command' attribute");

    let function_body = function.block;

    let alias_block = if alias.is_empty() {
        quote! {
            Vec::new()
        }
    } else {
        quote! {
            vec![#(#alias),*].into_iter().map(ToString::to_string).collect()
        }
    };

    let output = quote! {
        #[async_trait::async_trait(?Send)]
        impl Command<#ctx_type> for #target_struct {
            fn command_info(&self) -> CommandInfo {
                CommandInfo {
                    name: String::from(#name),
                    description: String::from(#description),
                    alias: #alias_block,
                }
            }

            async fn execute(&self, client: &Client<#ctx_type>, message: &PrivmsgMessage, args: &mut Args) -> Result<Box<dyn CommandResult<#ctx_type>>> {
                let result = #function_body;

                Ok(Box::new(result))
            }
        }
    };

    output.into()
}

#[proc_macro_attribute]
pub fn command_exec(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut impl_block = parse_macro_input!(item as ItemImpl);

    for item in &mut impl_block.items {
        if let ImplItem::Method(method) = item {
            if method.sig.ident == "execute" {
                // Change the return type of the function to `Result<Box<dyn CommandResult<Ctx>>>`
                method.sig.output = syn::parse2(quote! {
                    -> anyhow::Result<Box<dyn CommandResult<Ctx>>>
                })
                .unwrap();

                // Wrap the function body in a `Box::new`
                let old_body = &method.block;
                method.block = syn::parse2(quote! {
                    {
                        let result = #old_body;

                        Ok(Box::new(result))
                    }
                })
                .unwrap();
            }
        }
    }

    TokenStream::from(quote! {
        #[async_trait::async_trait(?Send)]
        #impl_block
    })
}
