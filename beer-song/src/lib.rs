pub fn verse(n: u32) -> String {
    match n {
        0 => "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".into(),
        1 => "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n".into(),
        _ => {
            let remaining_bottles = if n > 2 {
                format!("{} bottles", n - 1)
            } else /* n == 2 */ {
                "1 bottle".into()
            };
            format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} of beer on the wall.\n", n, n, remaining_bottles)
        }
    }
}

pub fn sing(start: u32, end: u32) -> String {
    let verses: Vec<String> = (end..=start).rev().map(verse).collect();
    verses.join("\n")
}
