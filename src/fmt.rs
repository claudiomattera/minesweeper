// Copyright Claudio Mattera 2021.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Text formatting primitives

pub fn pad_text(text: &str, length: usize) -> String {
    let mut text = text.to_owned();
    for _ in text.len()..length {
        text.push(' ');
    }
    text
}

pub fn format_number(mut n: u32, length: Option<usize>) -> String {
    let capacity = length.unwrap_or(10);
    let mut digits = Vec::with_capacity(capacity);
    while n > 0 {
        let digit = n % 10;
        n /= 10;
        digits.push(digit);
    }
    digits.reverse();

    let mut output = String::with_capacity(capacity);
    for _ in digits.len()..length.unwrap_or(0) {
        output.push(' ');
    }
    for digit in &digits {
        let c = char::from_u32(*digit as u32 + '0' as u32).unwrap();
        output.push(c);
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pad_text() {
        let input = "something";
        let text = pad_text(input, 20);
        let expected = "something           ";
        assert_eq!(text, expected);
    }

    #[test]
    fn test_format_number_5() {
        let n = 5;
        let text = format_number(n, Some(5));
        let expected = "    5";
        assert_eq!(text, expected);
    }

    #[test]
    fn test_format_number_23() {
        let n = 23;
        let text = format_number(n, Some(5));
        let expected = "   23";
        assert_eq!(text, expected);
    }

    #[test]
    fn test_format_number_100() {
        let n = 100;
        let text = format_number(n, Some(5));
        let expected = "  100";
        assert_eq!(text, expected);
    }
}
