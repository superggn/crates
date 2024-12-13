use regex::Regex;

fn main() {
    let re =
        Regex::new(r"(?P<group1>[a-z]*)-(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2})").unwrap();
    let caps = re.captures("haha-2010-03-14").unwrap();
    println!("caps: {:?}", caps);
    assert_eq!("haha", &caps["group1"]);
    assert_eq!("2010", &caps["year"]);
    assert_eq!("03", &caps["month"]);
    assert_eq!("14", &caps["day"]);
}
