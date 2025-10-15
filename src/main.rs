use std::fs;

fn main() {
    //variables
    let x: i32 = 883;
    let y: u32 = 1;
    let float: f32 = 1.1;
    print!("float {}", float);
    print!("y value {}", y);
    print!("x value {}", x);
    print!("Hello, world!");

    //booleans

    let is_male: bool = true;
    let mut is_female: bool = false;

    if is_female {
        println!("yes its female ");
    } else {
        println!("yes its male");
    }

    is_female = true;
    if is_female && is_male {
        println!("both gender are valid");
    }

    //string
    let greeting = String::from("First string");
    println!("Greetings: {}", greeting);

    let char1 = greeting.chars().nth(2);

    match char1 {
        Some(c) => println!("{}", c),
        None => println!("no chars at nth(100)"),
    }

    //loops

    fn lp() {
        for i in 0..11 {
            println!("i vale: {}", i);
        }
    }
    lp();

    let str1 = String::from("Hello this is the string to get the word");
    let str_ans = get_first_word(str1);
    println!("first word {}", str_ans);

    fn get_first_word(sentense: String) -> String {
        let mut ans = String::from("");
        for char in sentense.chars() {
            ans.push_str(char.to_string().as_str());
            if char == ' ' {
                break;
            }
        }
        return ans;
    }

    //function

    let a: i32 = 12;
    let b: i32 = 32;
    let s = sum(a, b);
    println!("{}", s);
    fn sum(a: i32, b: i32) -> i32 {
        return a + b;
    }

    //memoery management
    // mutable

    let mut muta = String::from("hello");
    muta.push_str("jayesh bro well done");
    println!("mutable {}", muta);
    //stack and heap

    fn one() {
        let num = 21;
        println!("function one");
        second(num);
    }
    fn second(num: i32) {
        println!("function two");
        println!("{}", num)
    }
    one();

    fn main_fn() {
        stack_fn();
        heap_fn();
        update_string();
    }
    fn stack_fn() {
        let a: i32 = 21;
        let b: i32 = 12;
        let c: i32 = a + b;
        println!("stack function: The sum of {} and {} is {}", a, b, c)
    }
    fn heap_fn() {
        let s1 = String::from("This is first string");
        let s2 = String::from("This is second string!");
        let combine = format!("{} {}", s1, s2);
        println!("Combine: {}", combine);
    }
    fn update_string() {
        let mut s1 = String::from("update");
        println!("Before Update: {}", s1);
        println!(
            "capacity: {} , Length: {} , pointer: {:p}",
            s1.capacity(),
            s1.len(),
            s1.as_ptr()
        );
        s1.push_str(" additional string data to store so capacity some how changes");
        println!("updated string: {}", s1);
        println!(
            "capacity: {} , Length: {} , pointer: {:p}",
            s1.capacity(),
            s1.len(),
            s1.as_ptr()
        );
    }

    main_fn();

    //ownership in rust

    //Example1:passing stack variables inside functions
    fn fn_1() {
        let x = 5;
        let y = 5;
        println!("{}", sum(x, y));
    }
    fn_1();

    fn scope_fn() {
        let x = 12;
        {
            let y = 21;
            println!("inside y scope: {}", y)
        }
        println!(" x level scope: {}", x);
    }
    scope_fn();

    fn rhiana() {
        let s1 = String::from("Hello i am rhiana's  boyfriend");
        let s2 = s1;
        println!("{}", s2); //If i try to print s1 value here it wont complie and give me error for borrowed value:
    }

    rhiana();

    let my_string = String::from("my string");
    take_myownersip(my_string);
    //  println!("{}",my_string); //This line would couse a compile error because ownership has been moved

    fn take_myownersip(some_string: String) {
        println!("{}", some_string); // `some_string` owns the data :
        //To work this out without cloning : we can return the string here like this: return some_string; and make my_string mutable, and take refrence as my_string = take_ownership(my_string); after that println!("{}",my_string works fine.) or we can introduce new variable calle my_string_3 = take_ownership(my_string) and print that.
    }

    //Borrowing and refrences

    fn borrow() {
        let mut s1 = String::from("hello");
        let s2 = &s1;
        println!("s2:{}", s2);
        println!("s1: {}", s1);
        borrow_ref(&s2);
        update_str(&mut s1);
    }
    borrow();
    fn borrow_ref(some_string: &String) {
        println!("{}", some_string);
    }
    fn update_str(s: &mut String) {
        s.push_str(" world");
        println!("{}", s);
    }

    //Structs
    struct User {
        active: bool,
        name: String,
        age: i32,
        email: String,
        count: u64,
    }
    let mut user1 = User {
        active: true,
        name: String::from("jayesh"),
        age: 21,
        email: String::from("jayesh@gmail.com"),
        count: 23,
    };
    println!(
        "The name is: {} , Status is: {} ,  Age: {}, Email: {}, count: {}",
        user1.name, user1.active, user1.age, user1.email, user1.count
    );
    user1.name.push_str(" khuman");
    println!("{}", user1.name);

    //Implementing structs
    struct Rect {
        height: u32,
        width: u32,
    }
    impl Rect {
        fn area(&self) -> u32 {
            self.height * self.width
        }
        fn perimeter(&self) -> u32 {
            2 * (self.height * self.width)
        }
    }
    let rect = Rect {
        height: 20,
        width: 20,
    };
    println!("Area is: {}", rect.area());
    println!("Perimeter {}", rect.perimeter());

    //enums and pattern matching

    enum Shape {
        Circle(f64),
        Square(f64),
        Reactangle(f64, f64),
    }
    fn calculate_area(shape: Shape) -> f64 {
        match shape {
            Shape::Circle(radius) => {
                println!("hey circle");
                3.14 * radius * radius
            }
            Shape::Square(side_length) => side_length * side_length,
            Shape::Reactangle(width, height) => width * height,
        }
    }
    let circle = Shape::Circle(5.0);
    let square = Shape::Square(10.0);
    let rectangle = Shape::Reactangle(12.00, 2.00);
    println!("Area of circle: {}", calculate_area(circle));
    println!("Area of square: {}", calculate_area(square));
    println!("Area of reactangle: {}", calculate_area(rectangle));

    //Error handling

    fn read_file() {
        let res = fs::read_to_string("example.txt");
        println!("hi there");
        match res {
            Ok(content) => {
                println!("File content: {}", content)
            }
            Err(err) => {
                println!("Error: {}", err)
            }
        }
        println!("hello");
    }
    read_file();

    //Option Enum

    fn finder_char_a(s: String) -> Option<i32> {
        for (index, character) in s.chars().enumerate() {
            if character == 'a' {
                return Some(index as i32);
            }
        }
        return None;
    }
    let the_string = String::from("my string awe");
    match finder_char_a(the_string) {
        Some(index) => println!("{}", index),
        None => println!("No a letter found in string"),
    }
}
