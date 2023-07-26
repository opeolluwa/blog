use regex::Regex;
fn main() {
    let valid_url = "https://docs.regex.rust";
    let invalid_url = "http:://.rg.o";

    let re = Regex::new(r"http(s)?:\/\/([\d\w]+)\.([\d\w]+)").expect("error parsing regex");

    println!("is the url {} valid? {}", valid_url, re.is_match(valid_url));
    // is the url https://docs.regex.rust valid? true

    println!(
        "is the url {} valid? {}",
        invalid_url,
        re.is_match(invalid_url)
    );
    //is the url http:://.rg.o valid? false
}
