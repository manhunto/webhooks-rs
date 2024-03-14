use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Attribute, DeriveInput, Expr, ExprLit, Lit, Meta};

#[proc_macro_derive(Ksuid, attributes(prefix))]
pub fn derive_ksuid_impl(item: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(item);

    let name = &input.ident;
    let name_str = name.to_string();

    let attrs: Vec<&Attribute> = input
        .attrs
        .iter()
        .filter(|attr| attr.path().is_ident("prefix"))
        .collect();

    if attrs.len() != 1 {
        panic!("a ksuid needs to be provided with the #[prefix = \"PREFIX\"] attribute");
    }

    let attr = attrs.first().unwrap();
    let prefix = match &attr.meta {
        Meta::NameValue(name_value) => match &name_value.value {
            Expr::Lit(ExprLit {
                lit: Lit::Str(string),
                ..
            }) => string.value(),
            _ => panic!("prefix attribute must be a string"),
        },
        _ => panic!("prefix attribute must be of the form `prefix = \"...\"`"),
    };

    let gen = quote! {
        impl #name {
            fn new() -> Self {
                Self {
                    id: format!("{}_{}", #prefix, Ksuid::new(None, None)),
                }
            }
        }

        impl Display for #name {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.id)
            }
        }

        impl TryFrom<String> for #name {
            type Error = String;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                let (prefix, _) = value.split_terminator('_').collect_tuple().unwrap();

                if prefix != #prefix {
                    return Err(format!(
                        "{} should have prefix {} but have {}",
                        #name_str, #prefix, prefix,
                    ));
                }

                Ok(Self { id: value })
            }
        }

    };
    gen.into()
}
