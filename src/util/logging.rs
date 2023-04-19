use crate::util::slice::Slice;

pub struct Logging {
}

impl Logging {

    // u64 append to the String
    #[inline]
    pub fn append_number_to(s: &mut String, num: u64) {
        s.push_str(&Logging::number_to_string(num));
    }

    // Slice append to the String
    #[inline]
    pub fn append_escaped_string_to(s: &mut String, value: &Slice) {
        s.push_str(&Logging::escape_string(value));
    }

    // u64 to String
    #[inline]
    pub fn number_to_string(num: u64) -> String {
        num.to_string()
    }

    // Slice to String
    #[inline]
    pub fn escape_string(value: &Slice) -> String {
        value.iter().map(|i| *i as char).collect::<String>()
    }

    // Slice to u64. on success return true and
    // advances "*sin" past the consumed number and
    // sets "*val" to the numeric value. Otherwise,
    // returns false and leaves *in in an unspecified state.
    pub fn consume_decimal_number(sin: &Slice) -> u64 {
        let mut value: u64 = 0;
        let max: u64 = u64::MAX;
        let last_u64_char: u64 = max % 10;

        let start = 0;
        let end = sin.size();
        let mut idx = 0;
        while idx < end {
            let ch = sin.get(idx).unwrap();
            if *ch < ('0' as u8) || *ch > ('9' as u8) {
                break;
            }
            let c: u64 = (*ch - '0' as u8) as u64;
            if value > max / 10 ||
                (value == max / 10 && c > last_u64_char) {
                return value;
            }
            value = value * 10 + c;
            idx += 1;
        }
        value
    }
}

