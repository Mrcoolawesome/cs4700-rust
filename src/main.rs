use std::{collections::VecDeque, fmt, usize};

/*
   Execution starts here
*/
fn main() {
    // run the tests
    test_pair_up();
    test_factorial_numbers();
    test_make_element_generator();
    test_run_length_encoding();
    test_forms_chain();
}

fn make_element_generator<T>(list: Vec<T>) -> impl FnMut() -> Option<T> {
    || None
}

/*
* Input: 1D Vector holding ints
* Output: 2D Vector holding vectors of length 2
* This function will take an input vector, and return a 2D vector containing
* the same numbers that the original vector had, but with the numbers in
* pairs within vectors of length 2.
*/
fn pair_up<T: Copy>(list: &Vec<T>) -> Vec<Vec<T>> {
    let mut return_vector: Vec<Vec<T>> = vec![];
    for i in 0..list.len() {
        // this does len() - 1 automatically
        if i + 1 < list.len() {
            return_vector.push(vec![list[i], list[i + 1]]);
        } else {
            return_vector.push(vec![list[i]]);
        }
    }

    return_vector
}

/*
* Input: Factorial limit to calculate to
* Output: Vector holding all the factorial numbers up to the given limit
* This function will take a number as a limit to calculate the factorial up to,
* and then will return all the factorial numbers until that number.
*/
fn factorial_numbers(n: i32) -> Vec<i32> {
    let mut return_vector: Vec<i32> = vec![];

    // recursive function that edits the return vector
    // if we put an & in front of 'mut', that makes that parameter pass by refrence (which is what
    // borrowing is in rust) and not by pass by value which is the default
    fn recurse_factorial(k: i32, list: &mut Vec<i32>) -> i32 {
        // return the current factorial value
        list.push(k);
        if k == 0 {
            return 1;
        }
        return k * recurse_factorial(k - 1, list);
    }
    // run the recurse_factorial function by letting the recursive function borrow it
    recurse_factorial(n, &mut return_vector);
    return return_vector;
}

// the whole 'T: Ord + Copy' bit is saying: it can be of any type but it must be orderable (Ord)
// and easily copyable without having to move data (hence the Copy part)
fn run_length_encoding<T: Ord + Copy>(list: &Vec<T>) -> Vec<(i32, T)> {
    // the first index of a pair is the num of occurances, second index is the actual number
    let mut return_vector: Vec<(i32, T)> = vec![]; // vector of tuples
    let mut used_nums: Vec<T> = vec![]; // the already visited numbers
    for i in list {
        if used_nums.contains(i) {
            continue; // skip this number if we've already done it
        }
        let mut curr_count: i32 = 0;
        for j in list {
            if j == i {
                curr_count += 1;
            }
        }
        // make sure to remember we did this number
        used_nums.push(*i);

        // add to the return vector
        return_vector.push((curr_count, *i));
        // You use * to derefrence a variable (so for example
        // it's no longer a pass by refrence type &T and now it's of type T because it's been
        // derefrenced)
    }

    return return_vector;
}

fn forms_chain<T: Ord>(list: &[Vec<T>]) -> bool {
    false
}

/*
  Function to test run_length_encoding
*/
fn test_run_length_encoding() {
    // Set up the test cases
    let test_cases = [vec![2, 3, 3], vec![2, 1, 2, 2, 2, 3], vec![], vec![77]];
    // Execute the test cases
    for test_case in test_cases {
        println!(
            "Run length encoding of {:?} is {:?}",
            test_case,
            run_length_encoding(&test_case)
        );
    }
}

/*
  Function to test forms_chain
*/
fn test_forms_chain() {
    // Set up the test cases
    let tests = [
        vec![],
        vec![vec![1, 2], vec![2, 1]],
        vec![vec![1, 2], vec![5, 3]],
        vec![vec![1, 5, 0], vec![0, 2, 3], vec![3, 2], vec![2, 7]],
    ];
    // Execute the test cases
    for test_case in tests {
        println!(
            "List is {:?} forms chain {} ",
            test_case,
            forms_chain(&test_case)
        );
    }
}

/*
  Function to test make_element_generator
*/
fn test_make_element_generator() {
    // Set up the test cases
    let tests = [vec![], vec![1, 2], vec![1, 2, 3]];
    // Execute the test cases
    for test_case in tests {
        let mut f = make_element_generator(test_case.clone());
        println!("Testing for vector {:?}", test_case);
        println!("  First element: {}", f().unwrap_or(0));
        println!("  Second element: {}", f().unwrap_or(0));
        println!("  Third number: {}", f().unwrap_or(0));
    }
}

/*
  Function to test pair_up
*/
fn test_pair_up() {
    // Set up the test cases
    let tests = [vec![], vec![1], vec![1, 2], vec![1, 2, 3, 4, 5]];
    // Execute the test cases
    for test_case in tests {
        println!("Pair up of {:?} is {:?}", test_case, pair_up(&test_case));
    }
}

/*
  Function to test factorial_numbers
*/
fn test_factorial_numbers() {
    println!("Factorials up to 1 {:?}", factorial_numbers(1));
    println!("Factorials up to 4 {:?}", factorial_numbers(4));
    println!("Factorials up to 10 {:?}", factorial_numbers(10));
}
