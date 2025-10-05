

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
    

    
}
