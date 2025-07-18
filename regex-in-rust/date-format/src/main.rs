use regex::Regex;

fn main() {
    let valid_date = "1999-25-05";
    let invalid_date = "05-25-1999";

    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").expect("error parsing regex");
    println!(
        "is the date \"{}\" format correct? {}",
        valid_date,
        re.is_match(valid_date)
    ); // is the date "1999-25-05" format correct? true

    println!(
        "is the date \"{}\" format correct? {}",
        invalid_date,
        re.is_match(invalid_date)
    ); // is the date is the date "05-25-1999" format correct? false
}
