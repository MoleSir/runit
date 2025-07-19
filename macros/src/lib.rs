use proc_macro_crate::{crate_name, FoundCrate};
use quote::quote;
use syn::{parse::Parse, parse_macro_input, Ident, LitFloat};

struct Input {
    value: LitFloat,
    suffix_unit: Ident,
}

impl Parse for Input {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let value = input.parse::<LitFloat>()?;
        let suffix_unit = input.parse::<Ident>()?;
        Ok(Input { value, suffix_unit })
    }
}

#[proc_macro]
pub fn u(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as Input);

    let s = input.suffix_unit.to_string();
    for (unit_name, unit) in UNITS_MAP {
        if s.ends_with(unit_name) {
            let suffix_len = s.len() - unit_name.len();
            let cpath = get_crate_path();
            let value = input.value.clone();
            let unit_ident = syn::Ident::new(unit, proc_macro2::Span::call_site());

            let code = if suffix_len != 0 {
                let suffix = syn::Ident::new(&s[..suffix_len], proc_macro2::Span::call_site());
                quote! {
                    #cpath::#unit_ident::new(#cpath::num!(#value #suffix))
                }
            } else {
                quote! {
                    #cpath::#unit_ident::new(#cpath::num!(#value))
                }
            };

            return code.into()
        }
    }

    let suffix_unit = input.suffix_unit;
    syn::Error::new_spanned(quote! { #suffix_unit }, "Invalid suffix+unit")
        .to_compile_error()
        .into()
}

fn get_crate_path() -> proc_macro2::TokenStream {
    match crate_name("runit") {
        Ok(FoundCrate::Itself) => {
            quote! { crate }
        }
        Ok(FoundCrate::Name(name)) => {
            let ident = syn::Ident::new(&name, proc_macro2::Span::call_site());
            quote! { #ident }
        }
        Err(_) => {
            quote! { runit }
        }
    }
}

const UNITS_MAP: &[(&str, &str)] = &[
    ("V", "Voltage"),
    ("v", "Voltage"),
    ("A", "Current"),
    ("Ω", "Resistance"),
    ("F", "Capacitance"),
    ("H", "Inductance"),
    ("Q", "Charge"),
    ("W", "Power"),
    ("J", "Energy"),
    ("s", "Time"),
    ("Hz", "Frequency"),
    ("HZ", "Frequency"),
    ("hz", "Frequency"),
    ("m", "Length"),
    ("m²", "Area"),
    ("N", "Force"),
    ("Pa", "Pressure"),
    ("Wb", "MagneticFlux"),
    ("T", "FluxDensity"),
    ("S", "Conductance"),
    ("m/s", "Velocity"),
    ("m/s²", "Accel"),
    ("K", "Temperature"),
    ("rad", "Angle"),
];
