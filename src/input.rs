use std::io;

fn main(){
    let mut input = String::new();

    println!("Enter: ");
    io::stdin().read_line(&mut input).expect("Cannot read!");
    
    println!("You was enter: {}",input);
    
        
}   
