use std::io;
use std::f64;
use indicatif::ProgressBar; //this crate should provide a progress bar
use indicatif::ProgressStyle; //this allows for styling
//use std::collections; cargo says I don't need this for Vec, idk

fn check(mut s: u128, n: u128, ip: u128) -> u128 {
    if (n % ip) == 0 { //this function should handle divisors higher than sqrt of the number
        s = s + ip;
    }
    return s;
}

fn main() {
    //large test number will be 2305843008139952128
    //larger test numbers exist, but are 10^18 times larger, and take days to compute
    //The square root of the 9th perfect number (which determines this algorithm's time complexity) is ~1,073,000,000 times larger
    //I estimate that the algorithm would take ~1900 years to determine if it is perfect
    //another test number is 137438691328
    //small test numbers are 6, 28, 496, and 8128
    //const n: u64 = 2305843008139952128;
    //the previous line was commented out because input is now taken

    let mut s = 0;
    let mut stack = Vec::new();
    let mut user_input = String::new();
    
    println!("Input the number: "); //output for input

    io::stdin().read_line(&mut user_input).expect("Failed to read input"); //input with exception for failure to read input

    user_input = user_input.trim().to_string(); //input trimmed to ensure it can be parsed to a u64
    //otherwise whitespace gets in the way
    //to_string() was used because trim produces a different kind of string than is required for parse()

    let n: u128 = user_input.parse().unwrap(); //changes string to u128

    let f = n as f64;
    let _root = f.sqrt(); //this square root massively reduces time complexity
    let iterations = _root.round() as u128;

    if n > 1000000000000000 { //this number should be roughly enough that loading feedback is required during execution
        let bar = ProgressBar::new(iterations as u64);
        //should display a loading bar

        bar.set_style(ProgressStyle::default_bar().template("[{percent}%] {wide_bar}").progress_chars("=>-"));
        //the line above displays the bar with a percent, which is more useful to users
        //it also has the bar fill the command line space

        println!("Note that loading bar will slow as program progresses"); //this alerts user to a necessary slowing of the loading bar
        for i in 1..iterations+1 {
            if (n % i) == 0 { //this checks for all of a number's divisors
                s = s + i; //this adds the divisor to the total
                stack.push(i);
                bar.inc((i/2) as u64); //i/2 produces accurate loading bar
                //printing within the if statement keeps the loading bar from massively slowing the program
                //printing out of iterations gives a realistic idea of execution time left
                //changed to print out of 1000 for easier to read output
            }
        }

        bar.finish();

        let bar0 = ProgressBar::new((stack.len() - 1) as u64);

        bar0.set_style(ProgressStyle::default_bar().template("[{percent}%] {wide_bar}").progress_chars("=>-"));

        while stack.len() > 1 {
            let enm = stack.pop();
            let popped = match enm { //match required because pop returns an enum
                Some(number) => number,
                None => 0,
            };

            let ip = n / popped;
            s = check(s, n, ip);

            bar0.inc(1);
        }

        bar0.finish();

        stack.pop(); //to fully empty the stack
        //you could also just count the final value then divide by 2, but I don't want to
    }

    else { //this does the for loop without the loading bar to prevent clutter for short execution times
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