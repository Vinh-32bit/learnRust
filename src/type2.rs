fn main(){
    //type tuy chinh
    struct Person{
        name: String,
        age: u32,
    }
    
    let person1 = Person{
        name: String::from("Vinh"),
        age: 19,
    };

    enum Direction {
        Up,
        Down,
        Left,
        Right,
    }

    let move_direction = Direction::Up;


    //tham chieu(reference.rs)
    let x: i32 = 2112005;
    let y = &x;
    println!("{},{}",x,y);

    let mut a: i32 = 1;
    let ref_a = &mut a;
    *ref_a += 1;
    println!("{}",a);

    //option:
    let value: Option<i32> = Some(10);
    let none: Option<String> = None;
    match value {
        Some(value) => println!("Value: {}", value),
        None => println!("value: None"),
    }

    match none {
        Some(none) => println!("none: {}", none),
        None => println!("value: None"),
    }

    //dynamic array (vector)
    let vec: Vec<i32> = vec![1,2,3,4,5];

    //HashMap
    use std::collections::HashMap;
    let mut info = HashMap::new();
    info.insert(String::from("Vinh"), 19);
    info.insert(String::from("Bob"), 17);
}