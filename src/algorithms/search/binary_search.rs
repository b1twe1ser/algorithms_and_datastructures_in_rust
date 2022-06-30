use std::cmp::Ordering;

pub fn binary_search<T: Ord>(items: &[T], target: &T) -> Option<usize> {
    let mut is_ascending = true;

    if items.len() > 1 {
        is_ascending = items[0] < items[items.len() - 1];
    }

    let mut right = items.len();
    let mut left = 0;

    while right > left {
        let mid = (right + left) / 2;
        if is_ascending {
            match target.cmp(&items[mid]) {
                Ordering::Less => right = mid,
                Ordering::Equal => return Some(mid),
                Ordering::Greater => left = mid + 1,
            }
        } else {
            match target.cmp(&items[mid]) {
                Ordering::Less => left = mid + 1,
                Ordering::Equal => return Some(mid),
                Ordering::Greater => right = mid,
            }
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
        let number = 4;

        let index = binary_search(&numbers, &number);
        assert_eq!(index, None);
    }

    #[test]
    fn end_index() {
        let numbers = [1, 2, 3, 4, 5];
        let number = 5;

        let index = binary_search(&numbers, &number);
        assert_eq!(index, Some(4));
    }

    #[test]
    fn first_index() {
        let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let number = 1;

        let index = binary_search(&numbers, &number);
        assert_eq!(index, Some(0));
    }

    #[test]
    fn first_index_letters() {
        let letters = ["a", "b", "c", "d", "e"];
        let letter = "a";

        let index = binary_search(&letters, &letter);
        assert_eq!(index, Some(0));
    }

    #[test]
    fn last_index_letter() {
        let letters = ["d", "e", "a", "j"];
        let letter = "j";

        let index = binary_search(&letters, &letter);
        assert_eq!(index, Some(3));
    }

    #[test]
    fn empty_letter() {
        let letters = [];
        let letter = "g";

        let index = binary_search(&letters, &letter);
        assert_eq!(index, None);
    }
}
