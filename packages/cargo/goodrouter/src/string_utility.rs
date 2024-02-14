use std::cmp;

pub fn find_common_prefix_length(chars_left: &Vec<char>, chars_right: &Vec<char>) -> usize {
    let common_length = cmp::min(chars_left.len(), chars_right.len());

    let mut index = 0;
    while index < common_length {
        let char_left = chars_left[index];
        let char_right = chars_right[index];
        if char_left != char_right {
            break;
        }

        index += 1;
    }

    index
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn common_prefix_length_test() {
        assert_eq!(
            find_common_prefix_length(
                &String::from("ab").chars().collect(),
                &String::from("abc").chars().collect()
            ),
            2
        );

        assert_eq!(
            find_common_prefix_length(
                &String::from("abc").chars().collect(),
                &String::from("abc").chars().collect()
            ),
            3
        );

        assert_eq!(
            find_common_prefix_length(
                &String::from("bc").chars().collect(),
                &String::from("abc").chars().collect()
            ),
            0,
        );
    }
}
