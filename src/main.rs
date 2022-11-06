#![deny(clippy::all)]

/*
 * hello
  ! hallo
  // hello
? hallo
TODO : hallo
*/
fn main() {
    let name: &str = "sofiane";
    let username: &str = "Gherab";
    let letter: char = 'é';

    let age: i32 = 24;

    println!(
        "Hello, world! ; my name is {} {} and I have  {} years old  , my favorite char is  {} ",
        name, username, age, letter
    );

    // ?  ============================================
    // * COURs 2
    // ?  ============================================

    // ? les variables mutable
    let mut number: u32 = 1;
    number += 1; // TODO : on elui ajouter +1

    // ? les pointeur
    let first_number: u32 = number;
    let first_number: u32 = first_number - 1;
    print!(" my first number id : {} ", first_number);

    // ? ============================================
    // * Rust : 3 - Types des variables
    // ?  ============================================

    /*
    * INTEGERS :
          ?  we can use :
                          i8 , i16 , i32 , i64 , i128
                          u8 , u16 , u32 , u64 , u128
                          iSize , uSize

          ?  signification :
                            *  i ->   ]-∞;+∞[
                            *  u ->   ]0;+∞[
                            * iSize -> array size ]-∞;+∞[
                            * uSize -> array size ]0;+∞[

     */
    let int_number: i32 = 123;
    let int_number_negative: i32 = -123;
    let this_number_is_positive: u32 = 123;

    print!(
        "{} -  {} - {} ",
        int_number, int_number_negative, this_number_is_positive
    );

    /*
    * FLOAT :
    ?  We can use :   f32 , f64
     */
    let pi1: f32 = 3.1415926;

    /*
     * BIOLEAN
     */
    let is_subscribe: bool = true;
    let is_not_sub: bool = false;

    /*
     * CHAR
     */
    let my_char = 'à';

    /*
     * STRING
     */
    let my_string: &str = "hello word";

    /*
     * TUPLE
     */
    // let (first: i32 , second : i32 , third : i32 ) =( 1,2,3 );
    let winner: (i32, i32, i32) = (1, 2, 3);
    let (fst, snd, trd) = (1, 2, 3);
    let the_fst = winner.0;

    /*
     * array
     */
    // let numbers_array: [u8; 5] = [0, 1, 2, 3, 4];
    let numbers_array: [u8; 5] = [1; 5];
    let array_length: usize = 4;
    let _the_last_el_of_array: u8 = numbers_array[4];
    // or
    let _the_last_el_of_array_2: u8 = numbers_array[array_length];

    /*
    * PARSING

     */

    let my_number_string: &str = "21";

    let _my_number_after_parsing = my_number_string
        .parse::<i32>() // on peut s'arreter là
        .expect("this is not a number"); // return an error of it's not number

    // * PARSE Des type proches en eux
    let array_2: [&str; 4] = ["hello"; 4];
    let index: i32 = 1;
    let index2: usize = index as usize;
    print!("{}", array_2[index as usize])

    //--
}
