use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut word_map: HashMap<&str, u16> = HashMap::new();
    note.iter().for_each(|&w| {
        word_map.entry(w).and_modify(|e| *e += 1).or_insert(1);
    });

    for w in magazine {
        if !word_map.contains_key(w) {
            continue;
        }

        if let Some(n) = word_map.get_mut(w) {
            if n > &mut 0 {
                *n -= 1;
            }
        }
    }

    word_map.values().sum::<u16>() == 0
}
