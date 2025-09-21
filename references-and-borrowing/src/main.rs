fn main() {
    let s1 = String::from("hello");

    let r1 = &s1;
    let r2 = &s1;

    println!("{r1} and {r2}");

    println!("{r1} and {r2}");

    let return_nothing = dangle();

    println!("{return_nothing}")
}

fn dangle() -> &String {
    let s = String::from("do something"); // you can't return a ref variable because it scope ends after the function ends.

    &s
}

// fn calculate_length(s: &String) -> usize {
//     // s is a reference to a string
//     s.len()
// } // here, s goes out of scope. But because it does not have ownership of what
// // it refers to, it is not dropped.

// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }
