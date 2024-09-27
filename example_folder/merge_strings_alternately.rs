// Define the Solution struct (it is empty but needed for the `impl` block).
// In Rust, even if a struct doesn't hold data, we use it to organize functions.
// if we where using the struct to hold data it would look like this:
// struct Solution {
//     data: i32,
// }
// impl Solution {
//     fn new(data: i32) -> Self {
//         Solution { data }
//     }
// }
// The Solution struct is used to group related functions together.
// those functions are actullay called methods in Rust.
struct Solution;

impl Solution {
    // This is a public function called `merge_alternately` which takes two input strings (`word1` and `word2`)
    // and returns a new string. The function merges characters from both input strings alternately.
    //
    // `word1` and `word2` are passed as `String` values, meaning they're owned by this function and
    // won't be accessible elsewhere after they're passed in. The function will return a new `String` 
    // that contains the merged characters.
    //
    // If one string is longer than the other, the remaining characters of the longer string will
    // be appended to the end of the result.
    //
    // Example usage:
    // let word1 = String::from("abc");
    // let word2 = String::from("defghi");
    // let result = Solution::merge_alternately(word1, word2);
    // Expected result: "adbecfghi"
    pub fn merge_alternately(word1: String, word2: String) -> String {
        // Create an empty String `result` to store the merged result.
        let mut result = String::new();
        
        // Convert `word1` and `word2` into iterators of characters using the `chars()` method.
        // This allows us to traverse each character of the strings one by one.
        let mut word1 = word1.chars();
        let mut word2 = word2.chars();

        // A loop that continues until both iterators are exhausted (i.e., no more characters left).
        loop {
            // `match` is used to handle the tuple of the next characters from both `word1` and `word2`.
            // Each `.next()` call returns an `Option`, either `Some(character)` or `None` if the iterator is exhausted.
            match (word1.next(), word2.next()) {
                // If both `word1` and `word2` still have characters (`Some(c1)` and `Some(c2)`),
                // we push them alternately into the `result` string.
                (Some(c1), Some(c2)) => {
                    result.push(c1);  // Add `c1` from `word1` to the result.
                    result.push(c2);  // Add `c2` from `word2` to the result.
                }
                // If `word1` has a character but `word2` is exhausted (`None`),
                // push the remaining characters from `word1` into the result.
                (Some(c1), None) => {
                    result.push(c1);  // Push the current character from `word1` into `result`.
                    // Continue pushing remaining characters from `word1` into the result.
                    while let Some(c1) = word1.next() {
                        result.push(c1);
                    }
                    break; // Break the loop since `word2` is exhausted.
                }
                // If `word2` has a character but `word1` is exhausted (`None`),
                // push the remaining characters from `word2` into the result.
                (None, Some(c2)) => {
                    result.push(c2);  // Push the current character from `word2` into `result`.
                    // Continue pushing remaining characters from `word2` into the result.
                    while let Some(c2) = word2.next() {
                        result.push(c2);
                    }
                    break; // Break the loop since `word1` is exhausted.
                }
                // If both `word1` and `word2` are exhausted (`None` from both),
                // the loop ends.
                (None, None) => break,
            }
        }

        // Return the final merged result as a String.
        result
    }
}

// The main function is the entry point of the program.
// It will create two strings and pass them to the `merge_alternately` method.
fn main() {
    // Create two example strings `word1` and `word2`.
    let word1 = String::from("abc");     // "abc"
    let word2 = String::from("defghi");  // "defghi"

    // Call the `merge_alternately` function from the `Solution` struct and store the result.
    let result = Solution::merge_alternately(word1, word2);

    // Print the result, which should merge both strings alternately.
    // Expected output: "adbecfghi"
    println!("{}", result);
}
