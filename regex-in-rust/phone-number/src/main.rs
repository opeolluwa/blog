use regex::Regex;

fn main() {
    let valid_phone = "+2340122863541";
    let invalid_phone = "+245090907468449";
    let re = Regex::new(r"^\+234\d{10}$").expect("error parsing regex");

    println!(
        "is the phone number \"{}\" valid {}",
        valid_phone,
        re.is_match(valid_phone)
    ); // is the phone number "+2340122863541" valid true

    println!(
        "is the phone number \"{}\" valid {}",
        invalid_phone,
        re.is_match(invalid_phone)
    ); //is the phone number "+245090907468449" valid false
}
