//! Mô phỏng ngữ nghĩa số/chuỗi của JavaScript (`Number(x)`, `x||y`, `String(x)`, `.slice`)
//! để dữ liệu save cũ (do JS ghi) được chuẩn hóa Y HỆT như trước.

use serde_json::Value;

/// Ký tự mà JS coi là khoảng trắng (`\s` và `trim()`).
pub fn is_js_space(c: char) -> bool {
    matches!(
        c,
        '\t' | '\n' | '\u{0B}' | '\u{0C}' | '\r' | ' ' | '\u{A0}' | '\u{1680}'
            | '\u{2000}'..='\u{200A}' | '\u{2028}' | '\u{2029}' | '\u{202F}' | '\u{205F}'
            | '\u{3000}' | '\u{FEFF}'
    )
}

pub fn js_trim(s: &str) -> &str {
    s.trim_matches(is_js_space)
}

fn radix(digits: &str, base: u32) -> f64 {
    if digits.is_empty() {
        return f64::NAN;
    }
    digits
        .chars()
        .try_fold(0f64, |acc, c| c.to_digit(base).map(|d| acc * base as f64 + d as f64))
        .unwrap_or(f64::NAN)
}

/// `Number("...")` của JS.
pub fn string_to_number(s: &str) -> f64 {
    let t = js_trim(s);
    if t.is_empty() {
        return 0.0;
    }
    match t {
        "Infinity" | "+Infinity" => return f64::INFINITY,
        "-Infinity" => return f64::NEG_INFINITY,
        _ => {}
    }
    for (p, base) in [("0x", 16), ("0X", 16), ("0o", 8), ("0O", 8), ("0b", 2), ("0B", 2)] {
        if let Some(rest) = t.strip_prefix(p) {
            return radix(rest, base);
        }
    }
    if t.chars().any(|c| !matches!(c, '0'..='9' | '.' | 'e' | 'E' | '+' | '-')) {
        return f64::NAN; // chặn "inf", "nan"... mà Rust chấp nhận còn JS thì không
    }
    t.parse::<f64>().unwrap_or(f64::NAN)
}

/// `Number(v)` của JS (v = undefined khi `None`).
pub fn to_number(v: Option<&Value>) -> f64 {
    match v {
        None => f64::NAN,
        Some(Value::Null) => 0.0,
        Some(Value::Bool(b)) => {
            if *b {
                1.0
            } else {
                0.0
            }
        }
        Some(Value::Number(n)) => n.as_f64().unwrap_or(f64::NAN),
        Some(Value::String(s)) => string_to_number(s),
        Some(Value::Array(a)) => match a.len() {
            0 => 0.0,
            1 => to_number(Some(&a[0])),
            _ => f64::NAN,
        },
        Some(Value::Object(_)) => f64::NAN,
    }
}

/// `Number(v) || fallback`.
pub fn num_or(v: Option<&Value>, fallback: f64) -> f64 {
    let n = to_number(v);
    if n.is_nan() || n == 0.0 {
        fallback
    } else {
        n
    }
}

/// Truthiness của JS.
pub fn truthy(v: Option<&Value>) -> bool {
    match v {
        None | Some(Value::Null) => false,
        Some(Value::Bool(b)) => *b,
        Some(Value::Number(n)) => n.as_f64().map(|x| x != 0.0 && !x.is_nan()).unwrap_or(false),
        Some(Value::String(s)) => !s.is_empty(),
        Some(_) => true,
    }
}

/// `Math.floor(n)` rồi chặn trong ±2^53 để đưa về i64 an toàn (NaN -> 0).
pub fn floor_i64(n: f64) -> i64 {
    const LIM: f64 = 9_007_199_254_740_992.0;
    if n.is_nan() {
        return 0;
    }
    n.floor().clamp(-LIM, LIM) as i64
}

pub fn number_to_string(n: f64) -> String {
    if n.is_nan() {
        "NaN".into()
    } else if n.is_infinite() {
        if n > 0.0 { "Infinity".into() } else { "-Infinity".into() }
    } else if n.fract() == 0.0 && n.abs() < 1e21 {
        format!("{}", n as i128)
    } else {
        format!("{}", n)
    }
}

/// `String(v)` của JS (gần đúng với mảng/đối tượng).
pub fn js_string(v: Option<&Value>) -> String {
    match v {
        None => "undefined".into(),
        Some(Value::Null) => "null".into(),
        Some(Value::Bool(b)) => b.to_string(),
        Some(Value::Number(n)) => number_to_string(n.as_f64().unwrap_or(f64::NAN)),
        Some(Value::String(s)) => s.clone(),
        Some(Value::Array(a)) => a
            .iter()
            .map(|x| if x.is_null() { String::new() } else { js_string(Some(x)) })
            .collect::<Vec<_>>()
            .join(","),
        Some(Value::Object(_)) => "[object Object]".into(),
    }
}

/// `str.slice(0, n)` theo đơn vị UTF-16 như JS (không cắt đôi một ký tự: bỏ nguyên ký tự vượt giới hạn).
pub fn slice_utf16(s: &str, n: usize) -> String {
    let mut used = 0usize;
    let mut out = String::new();
    for c in s.chars() {
        used += c.len_utf16();
        if used > n {
            break;
        }
        out.push(c);
    }
    out
}
