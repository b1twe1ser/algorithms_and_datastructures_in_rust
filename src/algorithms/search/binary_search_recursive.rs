use std::cmp::Ordering;

/// Finds the specified element inside a list of elements
///
/// # Arguments 🧩
///
/// * `items` the list of items of type `&[T]`
/// * `target` the specified element to find of type `&T`
/// * `left` the left most index of type `usize`
/// * `right` the right most index of type `usize`
///
/// # Returns ⏎
///
/// * `Option<usize>`
///
/// # Examples 👀
///
/// ```
/// let list_of_items = [1, 2, 3, 4, 5];
/// let item = 3;
///
/// let left = 0;
/// let right = list_of_items.len();
///
/// let index = linear_search_recursive(&list_of_items, &item, &left, &right);
/// assert_eq!(index, Some(2));
/// ```
pub fn binary_search_recursive<T: Ord>(
    items: &[T],
    target: &T,
    left: &usize,
    right: &usize,
) -> Option<usize> {
    if left >= right {
        return None;
    }

    let is_ascending = items[0] < items[items.len() - 1];

    let middle: usize = left + (right - left) / 2;

    if is_ascending {
        return match target.cmp(&items[middle]) {
            Ordering::Less => binary_search_recursive(items, target, left, &middle),
            Ordering::Greater => binary_search_recursive(items, target, &(middle + 1), right),
            Ordering::Equal => Some(middle),
        };
    } else {
        return match target.cmp(&items[middle]) {
            Ordering::Less => binary_search_recursive(items, target, &(middle + 1), right),
            Ordering::Greater => binary_search_recursive(items, target, left, &middle),
            Ordering::Equal => Some(middle),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const LEFT: usize = 0;

    #[test]
    fn fail_empty_list() {
        let list_of_items = vec![];
        assert_eq!(
            binary_search_recursive(&list_of_items, &1, &LEFT, &list_of_items.len()),
            None
        );
    }

    #[test]
    fn success_one_item() {
        let list_of_items = vec![30];
        assert_eq!(
            binary_search_recursive(&list_of_items, &30, &LEFT, &list_of_items.len()),
            Some(0)
        );
    }

    #[test]
    fn success_search_strings_asc() {
        let say_hello_list = vec!["hi", "olá", "salut"];
        let right = say_hello_list.len();
        assert_eq!(
            binary_search_recursive(&say_hello_list, &"hi", &LEFT, &right),
            Some(0)
        );
        assert_eq!(
            binary_search_recursive(&say_hello_list, &"salut", &LEFT, &right),
            Some(2)
        );
    }

    #[test]
    fn success_search_strings_desc() {
        let say_hello_list = vec!["salut", "olá", "hi"];
        let right = say_hello_list.len();
        assert_eq!(
            binary_search_recursive(&say_hello_list, &"hi", &LEFT, &right),
            Some(2)
        );
        assert_eq!(
            binary_search_recursive(&say_hello_list, &"salut", &LEFT, &right),
            Some(0)
        );
    }

    #[test]
    fn fail_search_strings_asc() {
        let say_hello_list = vec!["hi", "olá", "salut"];
        for target in &["adiós", "你好"] {
            assert_eq!(
                binary_search_recursive(&say_hello_list, target, &LEFT, &say_hello_list.len()),
                None
            );
        }
    }

    #[test]
    fn fail_search_strings_desc() {
        let say_hello_list = vec!["salut", "olá", "hi"];
        for target in &["adiós", "你好"] {
            assert_eq!(
                binary_search_recursive(&say_hello_list, target, &LEFT, &say_hello_list.len()),
                None
            );
        }
    }

    #[test]
    fn success_search_integers_asc() {
        let integers = vec![0, 10, 20, 30, 40, 50, 60, 70, 80, 90];
        for (index, target) in integers.iter().enumerate() {
            assert_eq!(
                binary_search_recursive(&integers, target, &LEFT, &integers.len()),
                Some(index)
            )
        }
    }

    #[test]
    fn success_search_integers_desc() {
        let integers = vec![90, 80, 70, 60, 50, 40, 30, 20, 10, 0];
        for (index, target) in integers.iter().enumerate() {
            assert_eq!(
                binary_search_recursive(&integers, target, &LEFT, &integers.len()),
                Some(index)
            )
        }
    }

    #[test]
    fn fail_search_integers() {
        let integers = vec![0, 10, 20, 30, 40, 50, 60, 70, 80, 90];
        for target in &[100, 444, 336] {
            assert_eq!(
                binary_search_recursive(&integers, target, &LEFT, &integers.len()),
                None
            );
        }
    }

    #[test]
    fn success_search_string_in_middle_of_unsorted_list() {
        let unsorted_strings = vec!["salut", "olá", "hi"];
        assert_eq!(
            binary_search_recursive(&unsorted_strings, &"olá", &LEFT, &unsorted_strings.len()),
            Some(1)
        );
    }

    #[test]
    fn success_search_integer_in_middle_of_unsorted_list() {
        let unsorted_integers = vec![90, 80, 70];
        assert_eq!(
            binary_search_recursive(&unsorted_integers, &80, &LEFT, &unsorted_integers.len()),
            Some(1)
        );
    }
}
