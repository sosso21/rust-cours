fn main() {
    // ? ========================
    // ?  cour 11 : ownerShip
    // ? ========================

    let name = String::from("Abdelbouboul");
    let clone_name = name; //.clone();
    println!(" my name is : {}", clone_name);

    // ? ========================
    // ?  cour 12 :  les ref
    // ? ========================
    let mut age = 10;
    let mut num = 12;

    let mut age2 = &mut age;

    println!("age1 :  , age2 {} ", age2);
    age2 = &mut num;

    println!("age1 :{} , age2 {}", age, age2);
}
