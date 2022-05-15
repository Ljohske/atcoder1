fn main() {
    let mut s = String::new();

    std::io::stdin().read_line(&mut s).expect("Failed readline");

    println!("You entered {}!", s);

    let strlen: usize = s.len();
    let mut estlen: usize = strlen;

    let mut ansstr = String::new();

    while estlen <= 6 {
        ansstr.push_str(&s);
        estlen += strlen;
    }

    println!("{}", ansstr);
}

fn lint_str(string_to_format: &str) -> String {
    let mut formatted_string = String::from(string_to_format);
    let boundary: usize = formatted_string.len() - 1;

    for idx in 1..boundary {
        formatted_string
    }
}
