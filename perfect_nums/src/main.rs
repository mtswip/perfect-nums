use std::io; //not sure this is necessary, but I can't tell now cause I'm testing a very large number
use std::f64;
use std::collections;

fn check(mut s: u64, n: u64, ip: u64, feedback: bool) -> u64 {
    if (n % ip) == 0 { //this function should handle divisors higher than sqrt of the number
        s = s + ip;
        if feedback {
            println!("Iteration {}/{}", ip, n);
        }
    }
    return s;
}

fn main() {
    //large test number will be 2305843008139952128
    //another test number is 137438691328
    //small test numbers are 6, 28, 496, and 8128
    const n: u64 = 137438691328; //eventually planning to change to mut u64
    let mut s = 0;
    let mut stack = Vec::new();
    
    //will worry about input later
    //println!("Input the number: ");

    //io::stdin()
        //.read_line(&mut n)
        //.expect("Failed to read input");

    let f = n as f64;
    let _root = f.sqrt();
    let iterations = _root.round() as u64;

    if n > 1000000 { //this number should be roughly enough that loading feedback is required during execution
        //should display a loading bar
        for i in 1..iterations+1 {
            if (n % i) == 0 { //this checks for all of a number's divisors
                s = s + i; //this adds the divisor to the total
                stack.push(i);
                println!("Iteration {}/{}", i, n); //I know this is a bad way to do this lol
                //printing within the if statement keeps it from printing way too many times
            }
        }

        while stack.len() > 1 {
            let enm = stack.pop();
            let popped = match enm { //match required because pop returns an enum
                Some(number) => number,
                None => 0,
            };
            let ip = n / popped;
            s = check(s, n, ip, true);
        }

        stack.pop(); //to fully empty the stack
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
            s = check(s, n, ip, false);
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