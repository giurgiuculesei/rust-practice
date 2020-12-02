#[test]
fn it_works() {
    assert_eq!(one_edit_away("abc", "a"), false);
    assert_eq!(one_edit_away("abc", "abd"), true);
    assert_eq!(one_edit_away("abc", "add"), false);
    assert_eq!(one_edit_away("abc", "aabr"), false);
    assert_eq!(one_edit_away("abc", "acv"), false);
    assert_eq!(one_edit_away("abc", "av"), false);
    assert_eq!(one_edit_away("abc", "ab"), true);
    assert_eq!(one_edit_away("abc", "ab"), true);
    assert_eq!(one_edit_away("abc", "axc"), true);
    assert_eq!(one_edit_away("abc", "abcd"), true);
    assert_eq!(one_edit_away("", ""), true);
}

fn one_edit_away(text1: &'static str, text2: &'static str) -> bool {
    //texts are too different case
    let length_distance = (text1.len() as i32 - text2.len() as i32).abs();
    //println!("length_distance {}", length_distance);

    if length_distance > 1 {
        return false;
    }

    // update case
    if text1.len() == text2.len() {
        let mut distance = 0;

        for i in 0..text1.len() {
            let char1 = text1.chars().nth(i).unwrap();
            let char2 = text2.chars().nth(i).unwrap();

            if char1 != char2 {
                distance = distance + 1
            }

            if distance > 1 {
                return false;
            }

        }
    }
    else { //insert or delete case
        let short_text = if text1.len() <= text2.len() { text1 } else { text2 };
        let long_text = if text1.len() <= text2.len() { text2 } else { text1 };

        for i in 0..short_text.len() {
            let char1 = short_text.chars().nth(i).unwrap();
            let char2 = long_text.chars().nth(i).unwrap();

            if char1 != char2 {
                return false;
            }
        }
    }

    return true;
}