use proc_macro::*;

fn target_feature(attr: TokenStream, enable: bool) -> String {
    let attrs: Vec<TokenTree> = attr.into_iter().collect();

    let mut output: String = String::new();

    for (index, attr) in attrs.iter().enumerate() {
        use TokenTree::{Ident, Punct};

        if index % 2 == 0 {
            if let Ident(feature) = attr {
                output += &format!(
                    "#[target_feature({}=\"{}\")]",
                    if enable { "enable" } else { "disable" },
                    {
                        #[cfg(any(target_arch="x86",target_arch="x86_64"))]
						match feature.to_string().as_ref() {
							"simd64_min" | "simd64" | "simd64_max" => panic!("Error: 64-bit SIMD (MMX) is currently unavailable, consider using 128-bit SIMD (SSE2)."),

							"simd128_min" => panic!("Warning: Minimal 128-bit SIMD (SSE) is unstable and unreliable, consider using recommended 128-bit SIMD (SSE2), \"simd128\"."),
							"simd128" => "sse2",
							"simd128_max" => "sse4.2",

							"simd256_min" | "simd256" => "avx",
							"simd256_max" => "avx2",

							"simd512_min" | "simd512" | "simd512_max" => "avx512",

							feature => panic!("Error: Unknown or unimplemented feature \"{}\" used!", feature),
						}
                    }
                );
            } else {
                panic!(
                    "Error: Expected feature identificator instead got \"{}\"!",
                    attr.to_string()
                );
            }
        } else if let Punct(punct) = attr {
            if punct.as_char() != ',' {
                panic!("Error: Expected ',' instead got '{}'!", punct.as_char());
            }
        } else {
            panic!("Error: Expected ',' instead got \"{}\"!", attr.to_string());
        }
    }

    output
}

#[proc_macro_attribute]
pub fn enable_feature(attr: TokenStream, item: TokenStream) -> TokenStream {
    format!("{}\n{}", target_feature(attr, true), item)
        .parse()
        .unwrap()
}

#[proc_macro_attribute]
pub fn disable_feature(attr: TokenStream, item: TokenStream) -> TokenStream {
    format!("{}\n{}", target_feature(attr, false), item)
        .parse()
        .unwrap()
}
