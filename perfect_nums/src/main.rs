use std::io; //not sure this is necessary, but I can't tell now cause I'm testing a very large number
use std::f64;
use std::collections;

fn check(mut s: u64, n: u64, ip: u64) -> u64 {
    if (n % ip) == 0 { //this function should handle divisors higher than sqrt of the number
        s = s + ip;
    }
    return s;
}

fn main() {
    //large test number will be 2305843008139952128
    //another test number is 137438691328
    //small test numbers are 6, 28, 496, and 8128
    //const n: u64 = 2305843008139952128;
    //the previous line was commented out because input is now taken

    let mut s = 0;
    let mut stack = Vec::new();
    let mut user_input = String::new();
    
    println!("Input the number: "); //output for input

    io::stdin()
        .read_line(&mut user_input) //input with exception for failure to read input
        .expect("Failed to read input");

    user_input = user_input.trim().to_string(); //input trimmed to ensure it can be parsed to a u64
    //otherwise whitespace gets in the way
    //to_string() was used because trim produces a different kind of string than is required for parse()

    let n: u64 = user_input.parse().unwrap(); //changes string to u64

    let f = n as f64;
    let _root = f.sqrt(); //this square root massively reduces time complexity
    let iterations = _root.round() as u64;

    if n > 1000000 { //this number should be roughly enough that loading feedback is required during execution
        //should display a loading bar
        for i in 1..iterations+1 {
            if (n % i) == 0 { //this checks for all of a number's divisors
                s = s + i; //this adds the divisor to the total
                stack.push(i);
                println!("Iteration {}/{}", i, iterations); //I know this is a bad way to do this lol
                //printing within the if statement keeps it from printing way too many times
                //printing out of iterations gives a realistic idea of execution time left
            }
        }

        while stack.len() > 1 {
            let enm = stack.pop();
            let popped = match enm { //match required because pop returns an enum
                Some(number) => number,
                None => 0,
            };
            let ip = n / popped;
            s = check(s, n, ip);
        }

        stack.pop(); //to fully empty the stack
        //you could also just count the final value then divide by 2, but I don't want to
    }

    else { //this does the for loop without the print statement to prevent clutter for short execution times
        for i in 1..iterations+1 {
            if (n % i) == 0 { //this checks for all of a number's divisors
                s = s + i; //this adds the divisor to the total
                stack.push(i); //this pushes to a stack for later
            }
        }

        while stack.len() > 1 {
            let enm = stack.pop();
            let popped = match enm {
                Some(number) => number,
                None => 0,
            };
            let ip = n / popped;
            s = check(s, n, ip);
        }

        stack.pop();
    }

    

    if s == n { //output for perfect numbers
        println!("That is a perfect number");
    }

    else { //output for non-perfect numbers
        println!("That is not a perfect number");
    }
}