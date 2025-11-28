use proc_macro::TokenStream;

mod id_parser;

#[proc_macro_derive(TemplateConfig, attributes(templateid))] // Must be at root
pub fn derive_template_config(input: TokenStream) -> TokenStream {
    id_parser::derive_template_config(input) // Delegate
}
