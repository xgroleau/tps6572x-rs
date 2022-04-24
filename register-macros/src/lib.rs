use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(RORegister)]
pub fn ro_register(input: TokenStream) -> TokenStream {
    // Parse the representation
    let ast = syn::parse(input).unwrap();

    // Build the impl
    let output = impl_ro_register(&ast);
    output.into()
}

fn impl_ro_register(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    quote! {
        #[allow(dead_code)]
        impl crate::registers::Register for #name {
            const ADDRESS: crate::registers::RegisterAddress = crate::registers::RegisterAddress::#name;
        }
    }
    .into()
}

#[proc_macro_derive(RWRegister)]
pub fn rw_register(input: TokenStream) -> TokenStream {
    // Parse the representation
    let ast = syn::parse(input).unwrap();

    // Build the impl
    let read = impl_ro_register(&ast);
    let write = impl_rw_register(&ast);

    let read_write = quote! {
        #read
        #write
    };
    read_write.into()
}
fn impl_rw_register(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
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
