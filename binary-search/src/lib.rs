pub fn find<T: Ord, U: AsRef<[T]>>(arr: U, key: T) -> Option<usize> {
    let array = arr.as_ref();
    if array.is_empty() {
        return None;
    }
    let mid = array.len() / 2;
    if array[mid] == key {
        return Some(mid);
    }
    if array[mid] > key {
        return find(array.get(..mid).unwrap(), key);
    }
    find(array.get(mid + 1..).unwrap(), key).map(|n| n + mid + 1)
}
