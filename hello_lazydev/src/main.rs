use std::collections::{hash_map, HashMap};

fn main() {
    let a: i32 = 10;
    let b: i32 = 15;
    println!("Hello, world!, {} {}", a,b);

    let character: char = 'c';
    println!("Character => {}", character);

    let boolean: bool = true;
    println!("Boolean => {}", boolean);

    let tuple: (i32, i32, f64, i32, bool) = (1, 2, 3.0, 4, true);
    println!("Tuple => {:?}", tuple);

    let array: [i32; 5] = [1,2,3,4,5];
    println!("Array => {:?}", array);

    let string: &str = "Hello World";
    println!("string => {}", string);

    let mut vector: Vec<i32> = vec![1, 2, 3, 4, 5];
    vector.push(6);
    println!("Vector => {:?}", vector);

    let mut hash_map: HashMap<&str, i32> = HashMap::new();
    hash_map.insert("Solana", 100);
    hash_map.insert("age", 2);
    println!("Hash Map => {:?}", hash_map);

    struct Point {
        x: f64,
        y: f64,
    }
}



                                                                                                                            