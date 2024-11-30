fn main(){
    let mut count: i32 = 0;

    loop{
        count += 1;
        println!("count:{}", count);

        if count >= 5{   
            break;
        }
         
    }
    // let mut number:u32 = 0;
    // while number <= 1000{
    //     println!("number {}", number);
    //     number += 1;
    // }

    let arr:[i32;8] = [1,2,3,4,5,6,7,8];

    for number in arr.iter(){
        println!("Number: {}", number);
    }

    let mut vec = vec![10,20,30,40,50,60];

    for value in vec {
        println!("Value: {}", value);
    }

    for i in 1..10 {
        print!(", {}", i); //1->9
    }
    println!();

    for i in 1..=5{
        println!("i: {}", i);
    }
}