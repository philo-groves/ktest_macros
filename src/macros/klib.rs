use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

pub fn klib(attr: TokenStream, item: TokenStream) -> TokenStream {
    let crate_name = if attr.is_empty() {
        quote! { env!("CARGO_PKG_NAME") }
    } else {
        let name = parse_macro_input!(attr as LitStr);
        quote! { #name }
    };
    
    let item_tokens = proc_macro2::TokenStream::from(item);
    
    // Don't emit inner attributes from the macro
    let output = quote! {
        #item_tokens

        #[cfg(test)]
        pub const BOOTLOADER_CONFIG: bootloader_api::BootloaderConfig = {
            let config = bootloader_api::BootloaderConfig::new_default();
            config
        };

        #[cfg(test)]
        #[panic_handler]
        fn panic(info: &core::panic::PanicInfo) -> ! {
            ktest::panic(info)
        }

        #[cfg(test)]
        fn kernel_test_main(_boot_info: &'static mut bootloader_api::BootInfo) -> ! {
            ktest::init_harness(#crate_name);
            test_main();
            loop {}
        }

        #[cfg(test)]
        bootloader_api::entry_point!(kernel_test_main, config = &BOOTLOADER_CONFIG);
    };

    output.into()
}
