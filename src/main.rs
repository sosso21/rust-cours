use std::collections::HashMap;
fn main() {
    // ? VECTORS
    let mut v1 = Vec::new();
    v1.push(1);
    v1.push(2);
    v1.push(3);

    let v2: Vec<i32> = vec![1, 2, 3, 4];

    let e2_v1: &i32 = &v1[1];
    let e2_v2: &i32 = &v2[1];

    match v1.get(3) {
        Some(third) => print!("third Element is {}", third),
        None => print!("there is no third element"),
    }

    println!(" {} , {}", e2_v1, e2_v2);

    let hello = String::from("ⴰⵣⵓⵍ");
    let a = hello.chars().nth(0).unwrap();
    println!("a : {} ", a);
    // ========================
    // ? hash
    // ========================

    let mut subjcts = HashMap::new();
    subjcts.insert("Eng", "English");
    subjcts.insert("Hindi", "Hindi");
    subjcts.insert("Maths", "Mathematics");
    println!("{:?}", subjcts);

    let end = "Eng";
    let get_eng = subjcts.get(&end);

    println!("give me eng : {:?}", get_eng);
}
