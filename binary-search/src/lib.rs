fn subfind(array: &[i32], key: i32, left: usize, right: usize) -> Option<usize> {
    if right < left {
        return None;
    }
    let mid = (right - left) / 2 + left;
    eprintln!("left = {}, right = {}, mid = {}", left, right, mid);
    if array[mid] == key {
        return Some(mid);
    }
    if array[mid] > key {
        if mid == 0 {
            // prevent trying to put negative in usize (mid-1 below)
            return None;
        }
        return subfind(array, key, left, mid - 1);
    }
    return subfind(array, key, mid + 1, right);
}

pub fn find(array: &[i32], key: i32) -> Option<usize> {
    if array.is_empty() {
        return None;
    }
    subfind(array, key, 0, array.len() - 1)
}
