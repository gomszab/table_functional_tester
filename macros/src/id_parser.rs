// myproj-macros/src/lib.rs
extern crate proc_macro;
use quote:: quote;
use syn::{Attribute, DeriveInput, Error, Fields, GenericArgument, LitStr, PathArguments, Type, parse_macro_input};
use syn::Ident;

pub fn derive_template_config(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let generics = &input.generics;

    let mut replacements = vec![];
    let mut errors = vec![];

    let data_struct = match input.data {
        syn::Data::Struct(data_struct) => data_struct,
        _ => {
            return Error::new_spanned(&input.ident, "Expected a struct")
                .to_compile_error()
                .into();
        }
    };

    if let Fields::Named(fields) = data_struct.fields {
        for field in fields.named {
            let field_name = field.ident.unwrap();
            let field_ty = field.ty;

            // Find mymacro attribute
            let macro_value = field
                .attrs
                .iter()
                .filter_map(|attr| {
                    if attr.path().is_ident("templateid") {
                        parse_macro_value(attr)
                    } else {
                        None
                    }
                })
                .next();


            let Some(value) = macro_value else {
                errors.push(Error::new_spanned(
                    &field_name,
                    "missing #[templateid(value = \"NAME\")]",
                ));
                continue;
            };

            let replacement = generate_replacement(&field_name, &field_ty, value);
            replacements.push(replacement);
        }
    }

    let error_tokens = errors.into_iter().map(|e| e.to_compile_error());
    let replacements_tokens = quote! {
        #( #error_tokens )*
        #(#replacements)*
    };

    let output = quote! {
        impl #generics #name #generics {
            pub fn wrap_id(&self, template: String) -> String {
                let mut template = template;
                #replacements_tokens
                template
            }
        }
    };
    output.into()
}

fn parse_macro_value(attr: &Attribute) -> Option<String> {
    if !attr.path().is_ident("templateid") {
        return None;
    }

    let mut captured = None;

    attr.parse_nested_meta(|meta| {
        if meta.path.is_ident("value"){
            let value = meta.value()?;
            let s: LitStr = value.parse()?;
            
            captured = Some(s.value());
        }
        Ok(())
    }).unwrap();
    captured
}

fn generate_replacement(
    field_name: &Ident,
    field_ty: &syn::Type,
    value: String,
) -> proc_macro2::TokenStream {
    let field_access = quote::quote!(self.#field_name);

    // Check field type and generate appropriate replacement
    match field_ty {
        syn::Type::Path(ty_path) if is_string_type(ty_path) => {
            quote! {
                template = template.replace(#value, &#field_access);
            }
        }
        syn::Type::Path(ty_path) if is_option_string_type(ty_path) => {
            quote! {
                template = template.replace(#value, &#field_access.clone().unwrap_or_else(|| "undefined".to_string()));
            }
        }
        syn::Type::Path(ty_path) if is_bool_type(ty_path) => {
            quote! {
                template = template.replace(#value, if #field_access { "true" } else { "false" });
            }
        }
        syn::Type::Path(ty_path) if is_option_bool_type(ty_path) => {
            quote! {
                template = template.replace(#value, &#field_access.map(|v| v.to_string()).unwrap_or_else(|| "false".to_string()));
            }
        }
        _ => quote! {
            // Unsupported type
        },
    }
}

fn is_string_type(ty_path: &syn::TypePath) -> bool {
    ty_path.path.segments.len() == 1 &&
    ty_path.path.is_ident("String")
}

fn is_bool_type(ty_path: &syn::TypePath) -> bool {
    ty_path.path.segments.len() == 1 &&
    ty_path.path.is_ident("bool")
}

fn is_option_string_type(ty_path: &syn::TypePath) -> bool {
    if let PathArguments::AngleBracketed(args) =  &ty_path.path.segments[0].arguments && let GenericArgument::Type(Type::Path(kk)) = &args.args[0] {
       return kk.path.is_ident("String");
    }
    false
}

fn is_option_bool_type(ty_path: &syn::TypePath) -> bool {
    if let PathArguments::AngleBracketed(args) =  &ty_path.path.segments[0].arguments && let GenericArgument::Type(Type::Path(kk)) = &args.args[0] {
       return kk.path.is_ident("bool");
    }
   false
}
