use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse, parse2, parse_str, Attribute, Error, Ident, ItemFn, Meta, ReturnType, Type};

pub fn ktest(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let function: ItemFn = match parse(item) {
        Ok(function) => function,
        Err(error) => return error.into_compile_error().into(),
    };
    let name = function.sig.ident.clone();
    let return_type = match &function.sig.output {
        ReturnType::Default => parse_str::<Type>("()").unwrap(),
        ReturnType::Type(_, return_type) => *return_type.clone(),
    };
    let attributes = match Attributes::try_from(&function.attrs) {
        Ok(attributes) => attributes,
        Err(error) => return error.into_compile_error().into(),
    };
    let ignore = attributes.ignore;
    let should_panic = attributes.should_panic;

    if return_type != parse_str::<Type>("()").unwrap()
        && should_panic != Ident::new("No", Span::call_site())
    {
        return Error::new_spanned(
            function,
            "functions using `#[should_panic]` must return `()`",
        )
        .into_compile_error()
        .into();
    }

    TokenStream::from(quote! {
        #[allow(dead_code)]
        #function

        #[test_case]
        #[allow(non_upper_case_globals)]
        const #name: ::ktest::test::Test::<#return_type> = ::ktest::test::Test::<#return_type> {
            name: stringify!(#name),
            modules: module_path!(),
            test: #name,
            ignore: ::ktest::test::Ignore::#ignore,
            should_panic: ::ktest::test::ShouldPanic::#should_panic,
        };
    })
}

struct Attributes {
    ignore: Ident,
    should_panic: Ident
}

impl Attributes {
    fn new() -> Self {
        Self {
            ignore: Ident::new("No", Span::call_site()),
            should_panic: Ident::new("No", Span::call_site())
        }
    }
}

impl TryFrom<&Vec<Attribute>> for Attributes {
    type Error = Error;

    fn try_from(attributes: &Vec<Attribute>) -> Result<Self, Self::Error> {
        let mut result = Attributes::new();

        for attribute in attributes {
            if let Some(ident) = attribute.path().get_ident() {
                match ident.to_string().as_str() {
                    "ignore" => {
                        match &attribute.meta {
                            Meta::NameValue(_name_value) => {
                                result.ignore = Ident::new("YesWithMessage", Span::call_site());
                            }
                            Meta::List(_) => return Err(Error::new_spanned(attribute, "valid forms for the attribute are `#[ignore]` and `#[ignore = \"reason\"]`")),
                            Meta::Path(_) => result.ignore = Ident::new("Yes", Span::call_site()),
                        }
                    }
                    "should_panic" => {
                        match &attribute.meta {
                            Meta::List(meta_list) => {
                                if let Ok(Meta::NameValue(name_value)) =
                                    parse2(meta_list.tokens.clone())
                                {
                                    if name_value.path == parse_str("expected").unwrap() {
                                        result.should_panic =
                                            Ident::new("YesWithMessage", Span::call_site());
                                    } else {
                                        return Err(Error::new_spanned(attribute, "argument must be of the form: `expected = \"error message\"`"));
                                    }
                                } else {
                                    return Err(Error::new_spanned(attribute, "argument must be of the form: `expected = \"error message\"`"));
                                }
                            }
                            Meta::NameValue(_name_value) => {
                                result.should_panic =
                                    Ident::new("YesWithMessage", Span::call_site());
                            }
                            Meta::Path(_) => {
                                result.should_panic = Ident::new("Yes", Span::call_site());
                            }
                        }
                    }
                    _ => {
                        // Not supported.
                    }
                }
            }
        }

        Ok(result)
    }
}
