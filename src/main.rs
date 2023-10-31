use merge_strings::merge_alternately;
fn main() {
    let s1: String = String::from("abc");
    let s2: String = String::from("pqrstu");
    let result = merge_alternately(s1, s2);
    println!("{}", result);
}
