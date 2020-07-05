pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return String::new();
    }
    let mut proverb = (0..list.len() - 1)
        .map(|i| format!("For want of a {} the {} was lost.", list[i], list[i + 1]))
        .collect::<Vec<String>>();
    proverb.push(format!("And all for the want of a {}.", list[0]));
    proverb.join("\n")
}
