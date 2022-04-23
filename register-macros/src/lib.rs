use proc_macro::TokenStream;
use quote::{quote, TokenStreamExt};
use syn;

#[proc_macro_derive(ReadRegister)]
pub fn read_register(input: TokenStream) -> TokenStream {
    // Parse the representation
    let ast = syn::parse(input).unwrap();

    // Build the impl
    impl_read_register(&ast)
}

fn impl_read_register(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    quote! {
        impl Register for #name {
            const ADDRESS: RegisterAddress = RegisterAddress::#name;
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

    // Concat read and write
    read.append(write);
    read
}
fn impl_write_register(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    quote! {
        impl WritableRegister for #name {}
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
