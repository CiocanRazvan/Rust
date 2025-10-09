//Problema 1
/*fn add_chars_n(mut s: String, c: char, n: u32) ->String{
    for _ in 0..=n
    {
        s.push(c);
    }
    return s;
}

fn main() {
    let mut s = String::from("");
    let mut i = 0;
    while i < 26 {
        let c = (i as u8 + b'a') as char;
        s = add_chars_n(s, c, 26 - i);

        i += 1;
    }

    print!("{}", s);
}*/

//Problema 2
/*fn add_chars_n(s: &mut String, c: char, n: u32){
    for _ in 0..=n
    {
        s.push(c);
    }
}

fn main() {
    let mut s = String::from("");
    let mut i = 0;
    while i < 26 {
        let c = (i as u8 + b'a') as char;
        add_chars_n(&mut s, c, 26 - i);

        i += 1;
    }

    print!("{}", s);
}*/

//Problema 3

fn add_str(s: &mut String, string: &str){
    s.push_str(&string);
}

fn add_space(s: &mut String, n: u32)
{
    for _ in 1..=n
    {
        s.push(' ');
    }
}

fn add_integer(s: &mut String, int: u32)
{
    let mut n = int;
    let mut i = 0;
    while n != 0
    {
        n = n / 10;
        i=i+1;
    }
    i = i%3;
    if i == 0
    {
        i = 3;
    }
    n = int;
    let mut ogl = 0;
    while n != 0
    {
        ogl = ogl*10 + n%10;
        n = n/10;
    }
    let mut j = 0;
    let mut i1 = i;
    while ogl != 0
    {
        if j == i1
        {
            s.push('_');
            i1=-1;
            j=0;
        }
        else if j == 3
        {
            s.push('_');
            j=0;
        }
        let a = ogl%10;
        let c = (a as u8 + b'0') as char;
        s.push(c);
        ogl = ogl /10;
        j = j+1;
    }
}
fn main(){
    let mut s = String::from("");
    add_str(&mut s, "Most");
    add_space(&mut s, 12);
    add_str(&mut s, "crate");
    add_space(&mut s, 7);
    add_integer(&mut s,34238908);
    add_space(&mut s, 12);
    add_str(&mut s, "and");
    add_space(&mut s, 7);
    add_str(&mut s, "lastest");
    add_space(&mut s, 12);
    add_str(&mut s, "is");
    add_space(&mut s, 12);
    add_str(&mut s, "downloaded");
    add_space(&mut s, 7);
    add_str(&mut s, "has");
    add_space(&mut s, 12);
    add_str(&mut s, "downloads");
    add_space(&mut s, 5);
    add_str(&mut s, "the");
    add_space(&mut s, 12);
    add_str(&mut s, "version");
    
    println!("{s}");
}
