use std::io; //not sure this is necessary, but I can't tell now cause I'm testing a very large number

fn main() {
    //large test number will be 2305843008139952128
    const n: u64 = 100000000; //eventually planning to change to mut u64
    let mut s = 0;
    
    //will worry about input later
    //println!("Input the number: ");

    //io::stdin()
        //.read_line(&mut n)
        //.expect("Failed to read input");

    if (n > 1000000) { //this number should be roughly enough that loading feedback is required during execution
        //should display a loading bar
        for i in 1..n {
            if (n % i) == 0 { //this checks for all of a number's divisors
                s = s + i; //this adds the divisor to the total
                println!("Iteration {}/{}", i, n); //I know this is a bad way to do this lol
                //printing within the if statement keeps it from printing way too many times
            }
        }
    }

    else { //this does the for loop without the print statement to prevent clutter for short execution times
        for i in 1..n {
            if (n % i) == 0 { //this checks for all of a number's divisors
                s = s + i; //this adds the divisor to the total
            }
        }
    }

    

    if s == n { //output for perfect numbers
        println!("That is a perfect number");
    }

    else { //output for non-perfect numbers
        println!("That is not a perfect number");
    }
}