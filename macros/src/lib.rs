extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use syn::LitStr;

#[proc_macro]
pub fn note(input: TokenStream) -> TokenStream {
    let input_str = parse_macro_input!(input as LitStr).value();

    let (name, octave_str) =
        input_str.split_at(if input_str.len() >= 2 && input_str.as_bytes()[1] == b'#' {
            2
        } else {
            1
        });

    let note_ident = match name {
        "C" => quote! { NoteName::C },
        "C#" => quote! { NoteName::CSharp },
        "D" => quote! { NoteName::D },
        "D#" => quote! { NoteName::DSharp },
        "E" => quote! { NoteName::E },
        "F" => quote! { NoteName::F },
        "F#" => quote! { NoteName::FSharp },
        "G" => quote! { NoteName::G },
        "G#" => quote! { NoteName::GSharp },
        "A" => quote! { NoteName::A },
        "A#" => quote! { NoteName::ASharp },
        "B" => quote! { NoteName::B },
        _ => panic!("Invalid note name: {}", name),
    };

    let octave: i32 = octave_str.parse().expect("Invalid octave number");

    TokenStream::from(quote! {
        Note::new(#octave, #note_ident)
    })
}
