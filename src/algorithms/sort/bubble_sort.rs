/// Sorts the elements inside a list
///
/// # Arguments 🧩
///
/// * `arr` the list_of_items to be sorted of type `&mut [T]`
///
/// # Returns ⏎
///
/// * `()`
///
/// # Examples 👀
///
/// ```
/// let mut list_of_items = [13, 5, 1,5];
///
/// bubble_sort(&mut list_of_items);
///
/// for i in 0..list_of_items.len() -1 {
/// assert!(list_of_items[i] <= list_of_items[i + 1])
/// }
///
/// ```
pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
    let mut sorted = false;
    let mut n = arr.len();
    while !sorted {
        sorted = true;
        for i in 0..n - 1 {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                sorted = false;
            }
        }
        n -= 1;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn descending() {
        //descending
        let mut ve1 = vec![6, 5, 4, 3, 2, 1];
        bubble_sort(&mut ve1);
        for i in 0..ve1.len() - 1 {
            assert!(ve1[i] <= ve1[i + 1]);
        }
    }

    #[test]
    fn ascending() {
        //pre-sorted
        let mut ve2 = vec![1, 2, 3, 4, 5, 6];
        bubble_sort(&mut ve2);
        for i in 0..ve2.len() - 1 {
            assert!(ve2[i] <= ve2[i + 1]);
        }
    }
}
