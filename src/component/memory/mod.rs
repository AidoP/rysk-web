use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "(number | undefined | null)")]
    pub type MaybeChar;
}
/// Get the expected length of the UTF-8 sequence from the first byte.
/// If the byte is not valid as a first UTF-8 byte [`None`] is returned.
const fn utf8_expected_len(b: u8) -> Option<u32> {
    if b & 0b1000_0000 == 0 {
        Some(1)
    } else {
        let ones = b.leading_ones();
        if ones < 2 || ones > 4 {
            None
        } else {
            Some(ones)
        }
    }
}
/// Get the significant part of a UTF-8 continuation byte, or [`None`]
/// if the byte is invalid for a UTF-8 continuation byte.
const fn utf8_continue_part(b: u8) -> Option<u8> {
    if b & 0b1100_0000 != 0b1000_0000 {
        None
    } else {
        Some(b & 0b0011_1111)
    }
}

/// Get the next char from the start of a slice.
fn decode_utf8_char(bytes: &[u8]) -> Option<char> {
    let a = *bytes.first()?;
    match utf8_expected_len(a)? {
        1 => char::from_u32((a & 0b0111_1111) as u32),
        2 => {
            let b = utf8_continue_part(*bytes.get(1)?)?;
            char::from_u32(((a & 0b0001_1111) as u32) << 6 | b as u32)
        }
        3 => {
            let b = utf8_continue_part(*bytes.get(1)?)?;
            let c = utf8_continue_part(*bytes.get(2)?)?;
            char::from_u32(((a & 0b0000_1111) as u32) << 12 | (b as u32) << 6 | c as u32)
        }
        4 => {
            let b = utf8_continue_part(*bytes.get(1)?)?;
            let c = utf8_continue_part(*bytes.get(2)?)?;
            let d = utf8_continue_part(*bytes.get(3)?)?;
            char::from_u32(
                ((a & 0b0000_0111) as u32) << 18 | (b as u32) << 12 | (c as u32) << 6 | d as u32,
            )
        }
        _ => unreachable!(),
    }
}

/// Try to decode a UTF-8 byte array to unicode points.
#[wasm_bindgen]
pub fn try_decode_bytes(bytes: &[u8]) -> Vec<MaybeChar> {
    let mut maybe_chars = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if let Some(char) = decode_utf8_char(&bytes[i..]) {
            let len = char.len_utf8();
            maybe_chars.push(JsValue::from(char as u32).into());
            for _ in 1..len {
                maybe_chars.push(JsValue::UNDEFINED.into());
            }
            i += len
        } else {
            maybe_chars.push(JsValue::NULL.into());
            i += 1
        }
    }
    maybe_chars
}
/// Get the UTF-8 length of a char for JS.
#[wasm_bindgen]
pub fn char_length(char: u32) -> usize {
    // If it isn't a char it should probably be treated as a byte
    char::from_u32(char).map(char::len_utf8).unwrap_or(1)
}

#[cfg(test)]
mod test {
    #[test]
    fn decode_utf8_char() {
        use super::decode_utf8_char as decode;
        const UTF8_DATA: &'static str = "§, 「Rocket 🚀」が好きな";
        let mut bytes = UTF8_DATA.as_bytes();
        for real in UTF8_DATA.chars() {
            let got = decode(bytes).expect("char should be valid");
            assert_eq!(real, got);
            bytes = &bytes[got.len_utf8()..];
        }
        assert!(bytes.is_empty());

        const RANDOM_DATA: &'static [u8] = &[
            b'a', 0xff, 0xc3, 0xe5, 0xc0, 0xe3, 0x81, 0x82, 0x20, 0xf0, 0x9f, 0x9a, 0x80,
        ];
        const EXPECTED: &'static [Option<char>] = &[
            Some('a'),
            None,
            None,
            None,
            None,
            Some('あ'),
            None,
            None,
            Some(' '),
            Some('🚀'),
            None,
            None,
            None,
        ];

        for i in 0..RANDOM_DATA.len() {
            assert_eq!(decode(&RANDOM_DATA[i..]), EXPECTED[i]);
        }
    }
}
