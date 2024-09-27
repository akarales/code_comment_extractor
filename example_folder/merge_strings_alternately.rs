struct Solution;
impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut result = String::new();
        let mut word1 = word1.chars();
        let mut word2 = word2.chars();
        loop {
            match (word1.next(), word2.next()) {
                (Some(c1), Some(c2)) => {
                    result.push(c1);
                    result.push(c2);
                }
                (Some(c1), None) => {
                    result.push(c1);
                    while let Some(c1) = word1.next() {
                        result.push(c1);
                    }
                    break;
                }
                (None, Some(c2)) => {
                    result.push(c2);
                    while let Some(c2) = word2.next() {
                        result.push(c2);
                    }
                    break;
                }
                (None, None) => break,
            }
        }
        result
    }
}
fn main() {
    let word1 = String::from("abc");
    let word2 = String::from("defghi");
    let result = Solution::merge_alternately(word1, word2);
    println!("{}", result);
}
