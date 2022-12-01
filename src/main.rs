fn main() {
    // ? ==============================
    // ? Cours 8
    // ? ==============================

    /*
    * Opntion
     @return   :  None | Some

    */

    let my_name: Option<String> = None;

    match my_name {
        Some(name) => println!("my name is  {}", name),
        None => println!("I have no name"),
    };

    let pont = Some(10);

    let ratio: f32 = match pont {
        Some(p) => {
            let ratio: f32 = p as f32 / 20 as f32;
            println!(" {} / {} = {} ", p, 20, ratio);
            // ? [1] we can return Some(ration)
            ratio
        }
        None => 0 as f32, // ? [1]  but we should return None
    };
    println!(" ratio {}", ratio);

    let age = 24;

    match age {
        0 => println!("tu viens de naitre "),  // if 0
        100 => println!("tu va creuver papi"), // if 100
        _ => println!("tva bosser"),           // = else
    };

    // ? ==============================
    // ? Cours 9 : les functions
    // ? ==============================

    // ? [2]  this function return nothing
    nothing(12);

    // ? [2]  this function return  hello
    let message = hello();
    println!("message : {}", message);
}

// ? [2] : -> () : mean that the function return nothing
fn nothing(x: i32) -> () {
    println!("{}", x)
}

fn hello() -> &'static str {
    return "hello";
}
