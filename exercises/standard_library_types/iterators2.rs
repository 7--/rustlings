// iterators2.rs
// In this exercise, you'll learn some of the unique advantages that iterators
// can offer. Follow the steps to complete the exercise.
// As always, there are hints if you execute `rustlings hint iterators2`!

// I AM NOT DONE

// Step 1.
// Complete the `capitalize_first` function.
// "hello" -> "HellO"
pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    let mut out = String::new();
    match c.next() {
        None => {},
        Some(first) => out.extend(first.to_uppercase()),
        //Some(last) => out.extend(last.to_uppercase()),
        //Some(any) =>  out.extend(any.to_lowercase()),
    }
    out
}

// Step 2.
// Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    
    let mut stringVecIter = words.iter();
    // while stringVecIter.hasNext(){
    //     //capitalize_first(stringVecIter.next())
    // }
    vec![]
}

// Step 3.
// Apply the `capitalize_first` function again to a slice of string slices.
// Return a single string.
// ["hello", " ", "world"] -> "Hello World"
pub fn capitalize_words_string(words: &[&str]) -> String {
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
