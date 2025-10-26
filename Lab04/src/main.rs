use std::fs;
use std::io;

fn problema1() -> Result<(), io::Error> {
    let s = fs::read_to_string("file.txt")?;

    let mut longest_bytes_line: Option<&str> = None;
    let mut longest_chars_line: Option<&str> = None;

    for line in s.lines() {
        longest_bytes_line = match longest_bytes_line {
            Some(prev) if prev.len() >= line.len() => Some(prev),
            _ => Some(line),
        };

        let line_chars_len = line.chars().count();
        longest_chars_line = match longest_chars_line {
            Some(prev) if prev.chars().count() >= line_chars_len => Some(prev),
            _ => Some(line),
        }
    }

    if let Some(line) = longest_bytes_line {
        println!("Longest line by bytes: {}", line);
    }
    println!();
    if let Some(line) = longest_chars_line {
        println!("Longest line by characters: {}", line);
    }

    Ok(())
}

fn problema2() -> Result<(), io::Error> {
    let s = fs::read_to_string("text.txt")?;

    for line in s.lines() {
        match rot13(line) {
            Ok(encoded) => {
                println!("ROT13 output: {}", encoded);
            }
            Err(err) => {
                eprintln!("Error: {}", err);
                break;
            }
        }
    }
    Ok(())
}

fn rot13(text: &str) -> Result<String, String> {
    let mut result = String::new();

    for c in text.chars() {
        if !c.is_ascii() {
            return Err(format!("Non-ASCII character encountered: '{}'", c));
        }

        let transformed = match c {
            'A'..='Z' => (((c as u8 - b'A' + 13) % 26) + b'A') as char,
            'a'..='z' => (((c as u8 - b'a' + 13) % 26) + b'a') as char,
            _ => c,
        };
        result.push(transformed);
    }
    Ok(result)
}

fn problema3() -> Result<(), io::Error> {
    let filename = "input.txt";
    let s = fs::read_to_string(filename)?;

    for line in s.lines() {
        let mut first = true;

        for word in line.split_whitespace() {
            let replace = match word {
                "pt" | "ptr" => "pentru",
                "dl" => "domnul",
                "dna" => "doamna",
                _ => word,
            };

            if !first {
                print!(" ");
            }
            first = false;

            print!("{}", replace);
        }
        println!();
    }

    Ok(())
}

fn problema4() -> Result<(), io::Error> {
    let path = r"C:\Windows\System32\drivers\etc\hosts";

    let contents = fs::read_to_string(path)?;

    for line in contents.lines() {
        let line = line.trim();

        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let mut parts = line.split_whitespace();

        if let (Some(hostname), Some(ip)) = (parts.next(), parts.next()) {
            println!("{} => {}", hostname, ip);
        }
    }

    Ok(())
}

fn main() -> Result<(), io::Error> {
    println!("Problema 1:\n");
    problema1()?;
    println!();
    println!("Problema 2:\n");
    problema2()?;
    println!();
    println!("Problema 3:\n");
    problema3()?;
    println!();
    println!("Problema 4:\n");
    problema4()?;
    Ok(())
}
