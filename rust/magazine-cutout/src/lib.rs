// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut notes_ocurrences_count: HashMap<&str, usize> = HashMap::new();

    let mut magazine_ocurrences_count: HashMap<&str, usize> = HashMap::new();

    let mut has_all_words: bool = false;

    for item in note {
        notes_ocurrences_count
            .entry(item)
            .and_modify(|word| {
                *word += 1;
            })
            .or_insert(1);
    }

    for item in magazine {
        magazine_ocurrences_count
            .entry(item)
            .and_modify(|word| {
                *word += 1;
            })
            .or_insert(1);
    }

    for (key, value) in notes_ocurrences_count {
        if magazine_ocurrences_count.contains_key(key) {
            has_all_words = magazine_ocurrences_count[key] >= value;

            if has_all_words == true {
                continue;
            } else {
                has_all_words == false;
                break;
            }
        } else {
            has_all_words = false;
            break;
        }
    }

    return has_all_words;
}