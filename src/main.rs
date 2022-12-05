fn main() {
    let s: String = String::from("Hello world!");
    let s2: &str = "salut";
    let a1 = [1, 2, 3];

    let r: &str = &s[0..5]; // from index 0 to 5
    let r1: &str = &s[6..]; // from index 6 to end
    let r2: &str = &s2[..]; // all
    let ra1 = &a1[..1]; // from index 0 to 1
    println!(
        " r: {} , r1 : {},  r2 : {} , ra1 : {} ",
        r,
        r1,
        r2,
        ra1.len()
    );
}
