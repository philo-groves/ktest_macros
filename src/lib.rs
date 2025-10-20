mod macros;

/// `#[ktest]` attribute macro
#[proc_macro_attribute]
pub fn ktest(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Due to Rust constraints, procedural macros must be defined in the root of the crate.
    // Therefore, we delegate the implementation to its interior `ktest` module.
    macros::ktest::ktest(attr, item)
}

/// `#[klib]` attribute macro
#[proc_macro_attribute]
pub fn klib(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Due to Rust constraints, procedural macros must be defined in the root of the crate.
    // Therefore, we delegate the implementation to its interior `klib` module.
    macros::klib::klib(attr, item)
}
