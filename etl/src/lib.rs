use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut map: BTreeMap<char, i32> = BTreeMap::new();
    for (score, letters) in h.iter() {
        for letter in letters {
            map.insert(letter.to_ascii_lowercase(), *score);
        }
    }
    return map;
}
