fn main() {
    let s1 = String::from("hello");

    let r1 = &s1;
    let r2 = &s1;

    println!("{r1} and {r2}");

    println!("{r1} and {r2}");

    // let return_nothing = dangle();

    // println!("{return_nothing}")

    let mut s = String::from("hello world");

    let word = first_word(&s);
    let word_slice = first_word_return_slice(&s);

    // let first_word = &s[0..word];
    // let remaining_word = &s[word..s.len()];

    println!("The first word is {:?}", word);
    println!("The slice is {}", word_slice);

    // println!("The reamining words is {remaining_word}");

    // println!("The length of the string is {word}");

    s.clear();
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    // for &item in bytes.iter() {
    //     if item == b' ' {
    //         return item;
    //     }
    // }

    s.len()
}

// fn dangle() -> &String {
//     let s = String::from("do something"); // you can't return a ref variable because it scope ends after the function ends.

//     &s
// }

// fn calculate_length(s: &String) -> usize {
//     // s is a reference to a string
//     s.len()
// } // here, s goes out of scope. But because it does not have ownership of what
// // it refers to, it is not dropped.

// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }

fn first_word_return_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
