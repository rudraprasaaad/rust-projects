fn main() {
    let x: i32 = 1;
    println!("x is {}", x);
    male_check("male", 23);

    say_greeting();
    odd_even(12);

    loops();
    let first_word = get_first_word(String::from("Rudraprasad Gouda"));
    println!("{}", first_word);
}

// here we &str in argument because it borrows String
// and don't take ownership of it the right of free
// the variable is still in the hands of the function
// who has ownership of it in this actual ownership of "male" is static.
fn male_check(is_male: &str, is_above_18: i8) {
    if is_male == "male" {
        println!("You are a male!")
    } else {
        println!("You are not a male")
    }

    if is_male == "male" && is_above_18 >= 18 {
        println!("You are a legal male")
    }
}

fn say_greeting() {
    let greeting = String::from("hello world");
    println!("{}", greeting);

    let greet = greeting.chars().nth(100);

    match greet {
        Some(c) => println!("{}", c),
        None => println!("No charatcter at that particular index."),
    }
}

fn odd_even(number: i32) {
    if number % 2 == 0 {
        println!("Number is even")
    } else {
        println!("Number is odd.")
    }
}

fn loops() {
    for i in 0..=10 {
        print!(" {} ", i)
    }
    println!()
}

fn get_first_word(sentence: String) -> String {
    let mut ans = String::from("");

    for char in sentence.chars() {
        ans.push_str(char.to_string().as_str());

        if char == ' ' {
            break;
        }
    }

    return ans;
}
