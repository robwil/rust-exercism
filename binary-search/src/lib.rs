use std::cmp::Ordering;

pub fn find<T: Ord, U: AsRef<[T]>>(arr: U, key: T) -> Option<usize> {
    let array = arr.as_ref();
    let mid = array.len() / 2;
    match key.cmp(array.get(mid)?) {
        Ordering::Equal => Some(mid),
        Ordering::Less => find(&array[..mid], key),
        Ordering::Greater => find(&array[mid + 1..], key).map(|n| n + mid + 1),
    }
}
