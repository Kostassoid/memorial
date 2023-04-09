use quote::{format_ident, quote};

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(FileParser, attributes(name))]
pub fn derive_file_parser(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = format_ident!("{}", input.ident.to_string());

    quote! {
        impl #struct_name {
            fn rule_to_quote(pair: Pair<Rule>) -> Quote {
                let line = pair.line_col().0;
                let indent = pair.line_col().1 - 1;

                /*@[Core/Parser]
                Handling of the indentations should be ideally done within the generated parser.
                But due to the lack of experience with Pest, this is done as a draft implementation
                using additional post-processing step. This would likely create additional challenge
                in case of multi-line comments using single-line syntax.
                 */
                let body = pair.into_inner().as_str()
                    .replace(&format!("\n{}", " ".repeat(indent)), "\n")
                    .replace(&format!("\n{}", "\t".repeat(indent)), "\n")
                    .trim()
                    .to_string();

                Quote {
                    line,
                    body,
                }
            }
        }

        #[allow(dead_code)]
        impl FileParser for #struct_name {
            fn parse_from_str(&self, source: &str) -> Result<Vec<Quote>> {
                let parsed = #struct_name::parse(Rule::root, source)?;
                Ok(parsed
                    .filter_map(|p| {
                        match p.as_rule() {
                            Rule::COMMENT => Some(#struct_name::rule_to_quote(p)),
                            _ => None
                        }
                    })
                    .collect())
            }
        }
    }
    .into()
}
