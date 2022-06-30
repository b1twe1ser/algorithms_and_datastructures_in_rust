use std::cmp::PartialEq;

/// Finds the specified element inside a list of elements
///
/// # Arguments 🧩
///
/// * `items` the list of items of type `&[T]`
/// * `target` the specified element to find of type `&T`
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
/// let index = linear_search(&list_of_items, &item);
/// assert_eq!(index, Some(2));
/// ```
pub fn linear_search<T: PartialEq>(items: &[T], target: &T) -> Option<usize> {
    for (index, value) in items.iter().enumerate() {
        if value == target {
            return Some(index);
        }
    }
    None
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn empty() {
        let numbers = [];
        let number = 2;

        let index = linear_search(&numbers, &number);
        assert_eq!(index, None);
    }

    #[test]
    fn end_index() {
        let numbers = [1, 2, 3, 4, 5];
        let number = 5;

        let index = linear_search(&numbers, &number);
        assert_eq!(index, Some(4));
    }

    #[test]
    fn first_index() {
        let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let number = 1;

        let index = linear_search(&numbers, &number);
        assert_eq!(index, Some(0));
    }

    #[test]
    fn first_index_letters() {
        let letters = ["a", "b", "c", "d", "e"];
        let letter = "a";

        let index = linear_search(&letters, &letter);
        assert_eq!(index, Some(0));
    }

    #[test]
    fn last_index_letter() {
        let letters = ["d", "e", "a", "j"];
        let letter = "j";

        let index = linear_search(&letters, &letter);
        assert_eq!(index, Some(3));
    }

    #[test]
    fn empty_letter() {
        let letters = [];
        let letter = "g";

        let index = linear_search(&letters, &letter);
        assert_eq!(index, None);
    }
}
