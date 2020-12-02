mod one_edit_away;
mod linked_list;
mod simple_linked_list;
mod pascal_triangle;
mod rectangles;

fn main() {
}

#[test]
fn it_works() {
    assert_eq!(check_for_uniqueness("abc").unwrap(), true);
    assert_eq!(check_for_uniqueness("abcb").unwrap(), false);
    assert_eq!(check_for_uniqueness("x").is_err(), true);
}
pub fn check_for_uniqueness(text: &str) -> Result<bool, &'static str> {
    let mut flags: u64 = 0;

    for letter in text.chars() {
        if let Some(i) = bit(letter) {
            if flags & i == i {
                return Ok(false);
            } else {
                flags = flags | i;
            }
        } else {
            return Err("Unknown character");
        }
    }

    return Ok(true);
}

fn bit(letter: char) -> Option<u64> {
    let base: u64 = 2;

    match letter {
        'a' => Some(base.pow(0)),
        'b' => Some(base.pow(1)),
        'c' => Some(base.pow(2)),
        'd' => Some(base.pow(3)),
        _ => None
    }
}