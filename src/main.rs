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

/*
 * Input: List of vectors with a generic type
 * Output: Function that returns either None or something of type T
 * This function will take an input vector, and reutrn a function that will return the next element
 * of the given vector for each time its ran.
 */
fn make_element_generator<T>(list: Vec<T>) -> impl FnMut() -> Option<T> {
    let mut list_copy: Vec<T> = list;
    // use the move keyword to put list_copy into the scope of the invoker function
    move || {
        if list_copy.is_empty() {
            // test cases use .unwrap_or(0) to turn None into 0 so don't need to worry about that
            return None; // need to return nothing if it's empty
        }

        let curr_element = list_copy.remove(0);
        return Some(curr_element); // need to say some to guarentee that it's not none
    }
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
        } else if i + 1 == list.len() && list.len() % 2 != 0 {
            // if the amount of elements is odd and if this is the final element
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
        if k == 0 {
            return 1;
        }
        let factorial_value = k * recurse_factorial(k - 1, list);
        list.push(factorial_value);
        return factorial_value;
    }
    // run the recurse_factorial function by letting the recursive function borrow it
    recurse_factorial(n, &mut return_vector);
    return return_vector;
}

/*
 * Input: a list with a generic type T that is orderable and copyable
 *      Note: copyable means it can be easily copied without moving data around
 * Output: Vector of tuples that say how many times a given element occured in the given list
 * This function will take in a list, and return a vector holding tuples that contain how many
 * instances of a given element were found in the given list.
 */
fn run_length_encoding<T: Ord + Copy>(list: &Vec<T>) -> Vec<(i32, T)> {
    // if the given vector is empty then return an empty vector
    if list.len() == 0 {
        return vec![];
    }
    // the first index of a pair is the num of occurances, second index is the actual number
    let mut return_vector: Vec<(i32, T)> = vec![]; // vector of tuples
    let mut prev_element: &T = &list[0]; // make the previous element the first in the given vector
    for i in 0..list.len() {
        let mut curr_count: i32 = 0;
        let mut keep_going: bool = true; // keep track of consecutiveness
        let mut j: usize = i; // usize is the type used to index lists
        let curr_element: &T = &list[i];

        // don't wanna make a new entry if we're still in a 'chain'.
        // this only applies to every element except the beginning since theres no previous to the
        // beginning element.
        if i > 0 && prev_element == curr_element {
            continue;
        }
        while keep_going {
            let temp_element: &T = &list[j];
            if temp_element == curr_element {
                curr_count += 1;
            } else {
                keep_going = false;
            }

            // check if we've reached the end of the list
            if j == list.len() - 1 {
                keep_going = false;
            }
            j += 1; // keep track of iteration
        }
        // add to the return vector
        return_vector.push((curr_count, *curr_element));
        // You use * to derefrence a variable (so for example
        // it's no longer a pass by refrence type &T and now it's of type T because it's been
        // derefrenced)

        // set the previous element to be the current one
        prev_element = curr_element;
    }

    return return_vector;
}

/*
 * Input: List of vector refrences that hold an orderable type.
 * Output: Boolean that returns true if the tail element of each vector in the list equals the head
 * element of the adjacent (i + 1) vector in the given list for each vector not adjacent to the end
 * of the list.
 * This function takes in a list of vectors that hold a comparible/orderable type and will return
 * true if the tail of each vector in the list equals the head element of the adjacent vector (the vector
 * in the i + 1 index) for each vector in the list excluding the vector at the end of the list.
 */
fn forms_chain<T: Ord>(list: &[Vec<T>]) -> bool {
    // does len() - 1 automatically
    for i in 0..list.len() {
        if i + 1 == list.len() {
            continue; // skip this iteration since there's no more chaining to be done
        }

        // we need to use a refrence to the vectors in the list to get them because otherwise we'd
        // be borrowing them and yanking them out of the memory that holds 'list'
        let curr_vector: &Vec<T> = &list[i];
        let next_vector: &Vec<T> = &list[i + 1];

        if curr_vector.last() != next_vector.first() {
            return false; // just immediately return false becuase a chain cannot be made
        }
    }

    // if we made it this far without returning false then a chain is possible
    return true;
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
