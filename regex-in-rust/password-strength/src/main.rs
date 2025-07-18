use regex::Regex;
fn main() {
    let strong_password = "_yt@308erjkDD-0$0!#";
    let weak_password = "aqaaaaaaa2233a";
    let re = Regex::new(r"([\w\W\D\d]+){8,}").expect("error parsing regex");
    println!("is the password strong {}", re.is_match(strong_password));
    println!("is the password strong {}", re.is_match(weak_password));
}
