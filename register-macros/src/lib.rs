use proc_macro::TokenStream;
use proc_macro2::TokenTree;
use quote::{quote, TokenStreamExt};
use syn;

#[proc_macro_derive(ReadRegister)]
pub fn read_register(input: TokenStream) -> TokenStream {
    // Parse the representation
    let ast = syn::parse(input).unwrap();

    // Build the impl
    let output = impl_read_register(&ast);
    output.into()
}

fn impl_read_register(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    quote! {
        #[allow(dead_code)]
        impl crate::registers::Register for #name {
            const ADDRESS: crate::registers::RegisterAddress = crate::registers::RegisterAddress::#name;
        }
    }
    .into()
}

#[proc_macro_derive(WriteRegister)]
pub fn write_register(input: TokenStream) -> TokenStream {
    // Parse the representation
    let ast = syn::parse(input).unwrap();

    // Build the impl
    let read = impl_read_register(&ast);
    let write = impl_write_register(&ast);

    let read_write = quote! {
        #read
        #write
    };
    read_write.into()
}
fn impl_write_register(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    quote! {
        #[allow(dead_code)]
        impl crate::registers::WritableRegister for #name {}
    }
    .into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
