fn main() {
    let num: i32 = 32;
    print_num(num);
    stack_fn();
    heap_fn();
    // update_string();
    let x = 1;
    let y = 23;
    println!("{}", sum(x, y));
    println!("{} {}", x, y);

    let i = 1;
    println!("{}", i);
    {
        let j = 3;
        println!("{}", j);
    }
    let my_string = String::from("cool");
    let my_string2 = my_string.clone();
    println!("{}", my_string2);
    takes_ownership(my_string);
}

fn print_num(num: i32) {
    println!("{}", num)
}

fn stack_fn() {
    let a = 10;
    let b = 20;
    let c = a + b;
    println!("Stack function: The sum of {} and {} is {}", a, b, c)
}

fn heap_fn() {
    let s1 = String::from("Hello");
    let s2 = String::from("World");
    let combined = format!("{} {}", s1, s2);
    println!("Heap function: Combined string is '{}'", combined)
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

// fn update_string() {
//     let mut s = String::from("Initial string");
//     println!("Before update: {}", s);
//     println!(
//         "Capacity: {}, Length: {}, pointer: {:p}",
//         s.capacity(),
//         s.len(),
//         s.as_ptr()
//     );

//     println!("After update: {}", s);
//     for _ in 0..100 {
//         s.push_str(" and some addtional text");
//         println!(
//             "Capacity: {}, Length: {}, pointer: {:p}",
//             s.capacity(),
//             s.len(),
//             s.as_ptr()
//         );
//     }
// }

fn sum(a: i32, b: i32) -> i32 {
    return a + b;
}
