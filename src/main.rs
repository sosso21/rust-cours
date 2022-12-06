#[derive(Debug)]
struct Color(i32, i32, i32);

#[derive(Debug)]
struct Point(i32, i32, i32);

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

impl User {
    fn create(person: Person) -> User {
        User {
            active: false,
            username: String::from(person.name),
            email: String::from("myEmail@gmail.com"),
            sign_in_count: 0,
        }
    }

    // This method is used to introduce a user
    fn introduction(&self) {
        println!(
            "Hello! My name is {} and My email is {}  .",
            self.username, self.email
        );
    }
}

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let black: Color = Color(0, 0, 0);
    let origin: Point = Point(0, 0, 0);

    let active = user1.active;
    let email = user1.email;
    let sign_in_count = user1.sign_in_count;

    let name = "Moh";
    let age = 27;
    let moh = Person { name, age };

    let moh_name = moh.name;
    let moh_age = moh.age;

    let change_user: User = User::create(moh);

    change_user.introduction();

    println!(
        "black : {black:?}  origin : {origin:?}  , user1 : {} , active : {active} email : {email} sign_in_count : {sign_in_count} , moh : {moh_name} moh age {moh_age} ",
        user1.username
    );
}
