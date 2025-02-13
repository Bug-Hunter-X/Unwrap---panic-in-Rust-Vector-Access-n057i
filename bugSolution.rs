fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    // Safe way to access the first element using match
    match vec.get(0) {
        Some(first) => println!("First element: {}", first),
        None => println!("Vector is empty!"),
    };

    //Alternative using expect for more informative error
    let first = vec.get(0).expect("Cannot get first element from an empty vector");
    println!("First element: {}", first);
} 