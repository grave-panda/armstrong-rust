use std::io;
use std::io::stdout;
use std::io::Write;


fn main() {
    print!("Enter a number: ");
    stdout().flush();

   	let mut input_text = String::new();
    io::stdin().read_line(&mut input_text).expect("failed to read from stdin");
    let trimmed = input_text.trim();
    match trimmed.parse::<i32>() {
        Ok(res) => {
            let mut x: i32 = res;
            let mut acc: i32 = 0;
            let pow: i32 = trimmed.len() as i32;
            while x > 0 {
                let d = x%10;
                x-=d;
                x/=10;
                acc+=d.pow(pow as u32);
            }
            if res==acc {
                println!("It is an armstrong number!");
            } else {
                println!("It is not an armstrong number!");
            }
        },
        Err(..) => {
            println!("this was not an integer: {}", trimmed);
        },
    };
}