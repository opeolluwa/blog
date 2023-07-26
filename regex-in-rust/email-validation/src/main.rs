use regex::Regex;

fn main() {
    let valid_email = "sampl44e-user@mailer.com";
    let invalid_email = "sample-user@mailer";
    let re = Regex::new(r"^[a-zA-Z0-9._-]+@[a-zA-Z0-9]+\.\w{2,}$")
        .expect("error parsing the regex syntax");

    println!(
        " is the email: {} valid? {}",
        valid_email,
        re.is_match(valid_email)
    ); // is the email sample44e-user@mailer.com valid? true

    println!(
        "is the email {} valid {}",
        invalid_email,
        re.is_match(invalid_email)
    ); // is the email sample-user@mailer valid false
}
