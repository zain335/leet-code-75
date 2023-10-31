use std::cmp;

pub fn merge_alternately(word1: String, word2: String) -> String {
    let word1_vec: Vec<char> = word1.chars().collect();
    let word2_vec: Vec<char> = word2.chars().collect();
    let len = word1_vec.len() + word2_vec.len();
    let mut result: Vec<char> = Vec::with_capacity(len);

    if !word1_vec.is_empty() && !word2_vec.is_empty() {
        for i in 0..cmp::max(word1_vec.len(), word2_vec.len()) {
            if i < word1_vec.len() {
                result.push(word1_vec[i % word1_vec.len()]);
            }
            if i < word2_vec.len() {
                result.push(word2_vec[i % word2_vec.len()]);
            }
        }
    }
    result.into_iter().collect()
}
