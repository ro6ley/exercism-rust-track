#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    let mut comparison = Comparison::Unequal;

    let f_len = first_list.len();
    let s_len = second_list.len();

    if f_len == s_len {
        // If the lengths match, they're either Equal or Unequal.
        if first_list == second_list {
            comparison = Comparison::Equal;
        }
    } else if f_len == 0 || s_len == 0 {
        comparison = if f_len == 0 { Comparison::Sublist } else { Comparison::Superlist };
    } else if f_len < s_len {
        // Scan over equal-length second_list slices to find a match.
        for n in 0..((s_len - f_len) + 1) {
            let s: usize = n as usize;
            let sl_slice = &second_list[s .. s + f_len];
            if sl_slice == first_list {
                comparison = Comparison::Sublist;
                break
            }
        }
    } else if s_len < f_len {
        // Scan over equal-length first_list slices to find a match.
        for n in 0..((f_len - s_len) + 1) {
            let s: usize = n as usize;
            let fl_slice = &first_list[s .. s + s_len];
            if fl_slice == second_list {
                comparison = Comparison::Superlist;
                break
            }
        }
    }    
    comparison
}
