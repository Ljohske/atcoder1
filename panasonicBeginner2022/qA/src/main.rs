fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("Failed readline");
    let s: String = s.trim().parse().expect("Parsing failed!");

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
