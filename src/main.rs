use std::env;


fn main() {
    let mut arguments = env::args();
    arguments.next(); 
  
    let mut reversed_string = String::from("");

    for mut argument in arguments {
        while let Some(c) = argument.pop(){
            reversed_string.push(c); 
        }
    }

    println!("{}", reversed_string);
}
