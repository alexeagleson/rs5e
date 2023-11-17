use proc_macro::TokenStream;
use quote::quote;
use std::ops::Not;

#[proc_macro_derive(Identifiable)]
pub fn identifiable_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_identifiable_macro(&ast)
}

fn impl_identifiable_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let lifetimes = ast.generics.lifetimes().collect::<Vec<_>>();

    let gen = match lifetimes.is_empty().not() {
        true => quote! {
            impl <#(#lifetimes)*> Identifiable for #name <#(#lifetimes)*> {
                fn id(&self) -> Id {
                    self.id
                }

                fn set_id(&mut self, new_id: Id) {
                    self.id = new_id;
                }
            }
        },
        false => quote! {
            impl  Identifiable for #name  {
                fn id(&self) -> Id {
                    self.id
                }

                fn set_id(&mut self, new_id: Id) {
                    self.id = new_id;
                }
            }
        },
    };
    gen.into()
}

#[proc_macro_derive(Named)]
pub fn named_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_named_macro(&ast)
}

fn impl_named_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Named for #name {
            fn name(&self) -> &str {
                self.name.as_str()
            }

            fn set_name(&mut self, new_name: String) {
                self.name = new_name;
            }
        }
    };
    gen.into()
}
