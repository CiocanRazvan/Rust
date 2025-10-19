use std::panic;

#[allow(dead_code)]
#[derive(Debug)]
enum ArithError {
    AddOver { a: u32, b: u32 },
    MulOver { a: u32, b: u32 },
}

enum FuncError {
    Ascii,
    Digit,
    HexDigit,
    Letter,
    Printable,
}

///Problema 1
fn prime(n: u16) -> bool {
    if n < 2 {
        return false;
    }

    for i in 2..=((n as f64).sqrt() as u16) {
        if n % i == 0 {
            return false;
        }
    }
    return true;
}

fn next_prime(x: u16) -> Option<u16> {
    let mut n = x + 1;
    while n < u16::MAX {
        if prime(n) {
            return Some(n);
        }
        n = n + 1;
    }
    None
}

///Problema 2
fn checked_add_u32(a: u32, b: u32) -> u32 {
    if u32::MAX - a < b {
        panic!("overflow: {a} + {b}");
    }
    a + b
}

fn checked_mul_u32(a: u32, b: u32) -> u32 {
    if a != 0 && b != 0 && a > u32::MAX / b {
        panic!("overflow: {a} * {b}");
    }
    a * b
}

///Problema 3
fn checked_add_u32_result(a: u32, b: u32) -> Result<u32, ArithError> {
    if u32::MAX - a < b {
        Err(ArithError::AddOver { a, b })
    } else {
        Ok(a + b)
    }
}

fn checked_mul_u32_result(a: u32, b: u32) -> Result<u32, ArithError> {
    if a != 0 && a > u32::MAX / b {
        Err(ArithError::MulOver { a, b })
    } else {
        Ok(a * b)
    }
}

fn add_then_mul(a: u32, b: u32, c: u32) -> Result<u32, ArithError> {
    let sum = checked_add_u32_result(a, b)?;
    checked_mul_u32_result(sum, c)
}

///Problema 4
fn to_uppercase(ch: char) -> Result<char, (FuncError, char)> {
    match ch {
        'a'..='z' => Ok(((ch as u8) - (b'a' - b'A')) as char),
        'A'..='Z' => Ok(ch),
        _ => Err((FuncError::Letter, ch)),
    }
}

fn to_lowercase(ch: char) -> Result<char, (FuncError, char)> {
    match ch {
        'a'..='z' => Ok(ch),
        'A'..='Z' => Ok(((ch as u8) + (b'a' - b'A')) as char),
        _ => Err((FuncError::Letter, ch)),
    }
}

fn print_char(ch: char) -> Result<char, (FuncError, char)> {
    match ch.is_ascii_graphic() {
        true => Ok(ch),
        false => Err((FuncError::Printable, ch)),
    }
}

fn char_to_number(ch: char) -> Result<u8, (FuncError, char)> {
    if !ch.is_ascii() {
        return Err((FuncError::Ascii, ch));
    }
    match ch {
        '0'..='9' => Ok(ch as u8 - b'0'),
        _ => Err((FuncError::Digit, ch)),
    }
}

fn char_to_number_hex(ch: char) -> Result<u8, (FuncError, char)> {
    if !ch.is_ascii() {
        return Err((FuncError::Ascii, ch));
    }
    match ch {
        '0'..='9' => Ok(ch as u8 - b'0'),
        'A'..='F' => Ok((ch as u8 - b'A') + 10u8),
        _ => Err((FuncError::HexDigit, ch)),
    }
}

fn print_error(eroare: (FuncError, char)) {
    match eroare.0 {
        FuncError::Ascii => println!("The characther '{}' is not ASCII.", eroare.1),
        FuncError::Digit => println!("The characther '{}' is not a digit", eroare.1),
        FuncError::HexDigit => println!("The characther '{}' is not a base 16 digit.", eroare.1),
        FuncError::Letter => println!("The characther '{}' is not a letter.", eroare.1),
        FuncError::Printable => println!("The characther {:?} is not printable.", eroare.1),
    }
}

fn main() {
    println!("Problema 1: ");

    let good: u16 = 30u16;
    let bad: u16 = u16::MAX;

    match next_prime(good) {
        Some(x) => println!("The next prime after {good} is {x}"),
        None => println!("There isn't a prime after {good} that fits in u16."),
    };

    match next_prime(bad) {
        Some(x) => println!("The next prime after {bad} is {x}"),
        None => println!("There isn't a prime after {bad} that fits in u16."),
    };

    println!("Problema 2:");

    let a = 68u32;
    let b = 31u32;
    let c = u32::MAX;

    let result = panic::catch_unwind(|| checked_add_u32(a, b));
    match result {
        Ok(res) => println!("The sum of {a} and {b} is {res:?}"),
        Err(_) => println!("Program panicked. The sum of {a} and {b} counldn't be computed."),
    }

    let result = panic::catch_unwind(|| checked_add_u32(b, c));
    match result {
        Ok(res) => println!("The sum of {b} and {c} is {res:?}"),
        Err(_) => println!("Program panicked. The sum of {b} and {c} counldn't be computed."),
    }

    let result = panic::catch_unwind(|| checked_mul_u32(a, b));
    match result {
        Ok(res) => println!("The multiplication of {a} and {b} is {res:?}."),
        Err(_) => {
            println!("Program panicked. The multiplication of {a} and {b} counldn't be computed")
        }
    }

    let result = panic::catch_unwind(|| checked_mul_u32(b, c));
    match result {
        Ok(res) => println!("The multiplication of {b} and {c} is {res:?}."),
        Err(_) => {
            println!("Program panicked. The multiplication of {b} and {c} counldn't be computed")
        }
    }

    println!("Problema 3:");

    match add_then_mul(40, 2, 1000) {
        Ok(v) => println!("(40 + 2) * 1000 = {v}"),
        Err(_) => eprintln!("Propagated error"),
    }

    match add_then_mul(u32::MAX, 0, 2) {
        Ok(v) => println!("val = {v}"),
        Err(_) => eprintln!("Propagated error"),
    }

    println!("Problema 4: ");

    match to_uppercase('z') {
        Ok(c) => println!("Uppercase of 'z' is '{c}'."),
        Err(e) => print_error(e),
    }
    match to_uppercase('X') {
        Ok(c) => println!("Uppercase of 'X' is '{c}'."),
        Err(e) => print_error(e),
    }
    match to_uppercase(',') {
        Ok(c) => println!("Uppercase of ',' is '{c}'."),
        Err(e) => print_error(e),
    }

    match to_lowercase('P') {
        Ok(c) => println!("Lowercase of 'P' is '{c}'."),
        Err(e) => print_error(e),
    }
    match to_lowercase('j') {
        Ok(c) => println!("Lowercase of 'j' is '{c}'."),
        Err(e) => print_error(e),
    }
    match to_lowercase('.') {
        Ok(c) => println!("Lowercase of '.' is '{c}'."),
        Err(e) => print_error(e),
    }

    match print_char('/') {
        Ok(c) => println!("Printable char: '{c}'."),
        Err(e) => print_error(e),
    }
    match print_char('\n') {
        Ok(c) => println!("Printable char: '{c}'."),
        Err(e) => print_error(e),
    }

    match char_to_number('5') {
        Ok(c) => println!("The conversion of '5' to digit is {c}."),
        Err(e) => print_error(e),
    }
    match char_to_number('x') {
        Ok(c) => println!("The conversion of '5' to digit is {c}."),
        Err(e) => print_error(e),
    }

    match char_to_number_hex('D') {
        Ok(c) => println!("The conversion of 'D' to base 16 digit is {c}."),
        Err(e) => print_error(e),
    }
    match char_to_number_hex('y') {
        Ok(c) => println!("The conversion of 'y' to base 16 digit is {c}."),
        Err(e) => print_error(e),
    }
    match char_to_number_hex('9') {
        Ok(c) => println!("The conversion of '9' to base 16 digit is {c}."),
        Err(e) => print_error(e),
    }
}
