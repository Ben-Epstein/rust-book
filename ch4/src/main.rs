fn main() {
    let mut first = String::from("Ferris");
    let full = add_suffix(&mut first);
    println!("{full}");
    println!("{first}");
    invalid_borrow();
    slices();
}

fn add_suffix(name: &mut String) -> String {
    name.push_str(" Jr.");
    return name.to_string();
}

fn invalid_borrow() {
    let mut v: Vec<i32> = vec![1, 2, 3];
    let num: &i32 = &v[2];
    // Rust knows that v.push happens _after_ num is no longer used (end of lifetime).
    //   if i move this println below v.push, ill get a borrow exception...
    println!("Third element is {}", *num);
    v.push(4);
}

// fn first_word(s: &str) -> &str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }

//     &s[..]
// }

fn print_type_of<T>(_: &T) {
    println!("type: {}", std::any::type_name::<T>())
}

fn slices() {
    let str_slice: &[&str] = &["hello", "world"]; // slice of string slices
    let string_slice: &[String] = &[
        "hello".to_string(), // converting string literals to owned Strings
        "world".to_string(),
    ];

    println!("String slice: {:?}", string_slice);
    print_type_of(&string_slice);
    println!("Str slice: {:?}", str_slice);
    print_type_of(&str_slice);
}
