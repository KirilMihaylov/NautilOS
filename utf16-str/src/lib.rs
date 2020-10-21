#![forbid(warnings, clippy::pedantic)]
#![feature(assoc_char_funcs)]

use proc_macro::TokenStream;

/// Function-like macro for converting Rust's UTF-8 string literals into UTF-16 encoded `u16` arrays.
#[proc_macro]
pub fn utf16(item: TokenStream) -> TokenStream {
    let literal: String = item.to_string();

    if !matches!(literal.as_bytes(), [b'"', .., b'"']) {
        panic!("Expected string literal!");
    }

    convert_to_utf16(&literal[1..literal.len() - 1]).parse().unwrap()
}

/// Function-like macro for converting Rust's UTF-8 string literals into UTF-16 encoded `u16` arrays with a terminating null (`\0`).
#[proc_macro]
pub fn c_utf16(item: TokenStream) -> TokenStream {
    let literal: String = item.to_string();

    if !matches!(literal.as_bytes(), [b'"', .., b'"']) {
        panic!("Expected string literal!");
    }

    convert_to_utf16(&format!("{}\0", &literal[1..literal.len() - 1])).parse().unwrap()
}

fn convert_to_utf16(s: &str) -> String {
    let mut return_str: String = String::new();

    let mut chars = s.chars();

    'chars: while let Some(ch) = chars.next() {
        if ch == '\\' {
            match chars.next() {
                Some('0') => return_str += "0u16,",
                Some(ch @ '\\') | Some(ch @ '\'') | Some(ch @ '"') => {
                    for value in ch.encode_utf16(&mut [0; 2]) {
                        return_str += &format!("{}u16, ", value);
                    }
                }
                Some(ch @ 'n') | Some(ch @ 't') | Some(ch @ 'r') => {
                    return_str += &format!("'\\{}' as u16,", ch)
                }
                Some('x') => match (chars.next(), chars.next()) {
                    (Some(ch1), Some(ch2))
                        if ch1.is_ascii_hexdigit() && ch2.is_ascii_hexdigit() =>
                    {
                        return_str += &format!("'\\x{}{}' as u16,", ch1, ch2)
                    }
                    _ => panic!("Malformed escape sequence!"),
                },
                Some('u') => {
                    const PANIC_MESSAGE: &str = "Malformed escape sequence!";

                    if let Some('{') = chars.next() {
                        let mut code: String = String::new();

                        let mut closing_bracket: bool = false;

                        'code: for _ in 0..6 {
                            match chars.next() {
                                Some(ch) if ch.is_ascii_hexdigit() => code.push(ch),
                                Some('}') => {
                                    closing_bracket = true;
                                    break 'code;
                                }
                                _ => panic!(PANIC_MESSAGE),
                            }
                        }

                        if !closing_bracket {
                            match chars.next() {
                                Some('}') => (),
                                _ => panic!(PANIC_MESSAGE),
                            }
                        }

                        for value in char::from_u32(
                            u32::from_str_radix(&code, 16).expect(PANIC_MESSAGE),
                        )
                        .expect(PANIC_MESSAGE)
                        .encode_utf16(&mut [0; 2])
                        {
                            return_str += &format!("{}u16,", value);
                        }

                        break 'chars;
                    }

                    panic!(PANIC_MESSAGE);
                }
                _ => panic!("Unknown escape code!"),
            }
        } else if ch == '"' {
            panic!("Unescaped double quotation!");
        } else {
            for value in ch.encode_utf16(&mut [0; 2]) {
                return_str += &format!("{}u16,", value);
            }
        }
    }

    format!("[{}]", return_str)
}
