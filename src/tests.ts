#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick_sort() {
        let mut arr = vec![4, 2, 5, 3, 1];
        quick_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_selection_sort() {
        let mut arr = vec![4, 2, 5, 3, 1];
        selection_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_insertion_sort() {
        let mut arr = vec![4, 2, 5, 3, 1];
        insertion_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_merge_sort() {
        let mut arr = vec![4, 2, 5, 3, 1];
        merge_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }
}
