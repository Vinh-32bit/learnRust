fn main(){
    println!("{}",plus(5,6));
    hello();
    error(0);
}

fn plus(a: i64, b: i64) -> i64 {
    a+b
}

fn hello() {
    println!("Hello")
}

fn error(num: i32) -> ! {
    if (num == 0) { 
        panic!("Zero Error");
    }
    unreachable!();
}