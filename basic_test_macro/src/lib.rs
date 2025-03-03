use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parse, parse_macro_input, Expr, Ident, PathArguments, Token, Type};

struct JsonTestData {
    json_value: Expr,
    var_name: Ident,
    var_type: Type,
}

impl Parse for JsonTestData {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let json_value = input.parse()?;
        let _comma: Token![,] = input.parse()?;
        let var_name = input.parse()?;
        let _comma: Token![,] = input.parse()?;
        let var_type = input.parse::<Type>()?;

        Ok(Self { json_value, var_name, var_type })
    }
}

#[proc_macro]
pub fn type_converter(input_tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input_tokens as Type);
    let expanded = match &input {
        Type::Path(type_path) => {
            if type_path.path.is_ident("String") {
                quote! {
                    |val: &serde_json::Value| -> String {
                        val.as_str().unwrap().to_string()
                    }
                }
            }
            else if type_path.path.is_ident("bool") {
                quote! {
                    |val: &serde_json::Value| -> bool {
                        val.as_bool().unwrap()
                    }
                }
            }
            else if type_path.path.is_ident("i32") {
                quote! {
                    |val: &serde_json::Value| -> i32 {
                        val.as_i64().unwrap() as i32
                    }
                }
            }
            else if type_path.path.is_ident("f64") {
                quote! {
                    |val: &serde_json::Value| -> f64 {
                        val.as_f64().unwrap()
                    }
                }
            }
            else if type_path.path.segments.first().unwrap().ident == "Vec" {
                let generic_args = &type_path.path.segments.first().unwrap().arguments;
                if let PathArguments::AngleBracketed(args) = generic_args {
                    let arg_count = args.args.len();
                    if arg_count != 1 {
                        panic!("Vector<T> must have exactly one generic arguments, found {}", arg_count);
                    }
    
                    let mut iter = args.args.iter();
                    let inner_type = iter.next().unwrap();
    
                    quote! {
                        |val: &serde_json::Value| -> Vec<#inner_type> {
                            val.as_array()
                                .unwrap()
                                .iter()
                                .map(|v| {
                                    let converter = type_converter!(#inner_type);
                                    converter(v)
                                })
                                .collect()
                        }
                    }
                } else {
                    panic!("Vector must have angle-bracketed generic arguments");
                }
            }
            else if type_path.path.segments.first().unwrap().ident == "HashMap" {
                let generic_args = &type_path.path.segments.first().unwrap().arguments;
                if let PathArguments::AngleBracketed(args) = generic_args {
                    let arg_count = args.args.len();
                    if arg_count != 2 {
                        panic!("HashMap<K, V> must have exactly two generic arguments, found {}", arg_count);
                    }
    
                    let mut iter = args.args.iter();
                    let key_type = iter.next().unwrap();
                    let value_type = iter.next().unwrap();
    
                    quote! {
                        |val: &serde_json::Value| -> std::collections::HashMap<#key_type, #value_type> {
                            val.as_object()
                                .unwrap()
                                .iter()
                                .map(|(key, value)| {
                                    let k_converter = type_converter!(#key_type);
                                    let v_converter = type_converter!(#value_type);
                                    (k_converter(&serde_json::Value::String(key.clone())), v_converter(value))
                                })
                                .collect()
                        }
                    }
                } else {
                    panic!("HashMap must have angle-bracketed generic arguments");
                }
            }
            else {
                panic!("Invalid type");
            }
        }
        // Type::Reference(type_ref) => {
        //     panic!("TypeReference not supported: {}", type_ref.elem;);
        // }
        _ => panic!("Unsupported type!")
    };

    expanded.into()
}


#[proc_macro]
pub fn extract_data(input: TokenStream) -> TokenStream {
    let JsonTestData { json_value, var_name, var_type } = parse_macro_input!(input as JsonTestData);

    let expanded = quote! {
        let converter = type_converter!(#var_type);
        let #var_name: #var_type = converter(&#json_value[stringify!(#var_name)]);
    };

    TokenStream::from(expanded)
}
