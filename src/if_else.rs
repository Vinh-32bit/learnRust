use std::io;
fn main(){
    let num: i32 = 0;
    
    if num < 0{
        println!("{} is nega", num);
    }
    else if num == 0{
        println!("{} is zero", num);

    }

    else{
        println!("{} is positive", num);
    }

    let age = 19;

    let result = if age <18{
        "Young"
    } else if age > 18{
        "Old"
    } else {
        "Enough"
    };
    println!("{}", result);

    //match (switch)
    let mut input = String::new();

    println!("Enter number: ");
    io::stdin().read_line(&mut input).expect("Cannot read");
    let num: i32 = input.trim().parse().expect("INT");

    match num {
        1 => println!("num 1"),
        2 => println!("num 2"),
        _ => println!("Other"),
    }

    let number = 1;
    let r = if number % 2 == 0 {"Even"} else {"Ood"};
    println!("{}", r);



    let number = 10;
    if number > 0 && number < 100 {
        println!("Số nằm trong khoảng từ 1 đến 99");
    } else if number < 0 || number > 100 {
        println!("Số nằm ngoài khoảng từ 1 đến 99");
    }

}