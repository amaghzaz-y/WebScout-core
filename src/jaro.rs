use alloc::vec::Vec;
use core::cmp::{max, min};
use core::fmt;
use core::fmt::{Display, Formatter};
use core::str::Chars;
#[derive(Debug, PartialEq)]
pub enum StrSimError {
    DifferentLengthArgs,
}

impl Display for StrSimError {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), fmt::Error> {
        let text = match self {
            StrSimError::DifferentLengthArgs => "Differing length arguments provided",
        };

        write!(fmt, "{}", text)
    }
}

pub fn generic_jaro<'a, 'b, Iter1, Iter2, Elem1, Elem2>(a: &'a Iter1, b: &'b Iter2) -> f32
where
    &'a Iter1: IntoIterator<Item = Elem1>,
    &'b Iter2: IntoIterator<Item = Elem2>,
    Elem1: PartialEq<Elem2>,
{
    let a_len = a.into_iter().count();
    let b_len = b.into_iter().count();
    if a_len == 0 && b_len == 0 {
        return 1.0;
    } else if a_len == 0 || b_len == 0 {
        return 0.0;
    } else if a_len == 1 && b_len == 1 {
        return if a.into_iter().eq(b.into_iter()) {
            1.0
        } else {
            0.0
        };
    }

    let search_range = (max(a_len, b_len) / 2) - 1;

    let mut b_consumed = Vec::with_capacity(b_len);
    for _ in 0..b_len {
        b_consumed.push(false);
    }
    let mut matches = 0.0;

    let mut transpositions = 0.0;
    let mut b_match_index = 0;

    for (i, a_elem) in a.into_iter().enumerate() {
        let min_bound = if i > search_range {
            max(0, i - search_range)
        } else {
            0
        };

        let max_bound = min(b_len - 1, i + search_range);

        if min_bound > max_bound {
            continue;
        }

        for (j, b_elem) in b.into_iter().enumerate() {
            if min_bound <= j && j <= max_bound && a_elem == b_elem && !b_consumed[j] {
                b_consumed[j] = true;
                matches += 1.0;

                if j < b_match_index {
                    transpositions += 1.0;
                }
                b_match_index = j;

                break;
            }
        }
    }

    if matches == 0.0 {
        0.0
    } else {
        (1.0 / 3.0)
            * ((matches / a_len as f32)
                + (matches / b_len as f32)
                + ((matches - transpositions) / matches))
    }
}

struct StringWrapper<'a>(&'a str);

impl<'a, 'b> IntoIterator for &'a StringWrapper<'b> {
    type Item = char;
    type IntoIter = Chars<'b>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.chars()
    }
}
pub fn jaro(a: &str, b: &str) -> f32 {
    generic_jaro(&StringWrapper(a), &StringWrapper(b))
}

pub fn generic_jaro_winkler<'a, 'b, Iter1, Iter2, Elem1, Elem2>(a: &'a Iter1, b: &'b Iter2) -> f32
where
    &'a Iter1: IntoIterator<Item = Elem1>,
    &'b Iter2: IntoIterator<Item = Elem2>,
    Elem1: PartialEq<Elem2>,
{
    let jaro_distance = generic_jaro(a, b);
    let prefix_length = a
        .into_iter()
        .zip(b.into_iter())
        .take_while(|&(ref a_elem, ref b_elem)| a_elem == b_elem)
        .count();

    let jaro_winkler_distance =
        jaro_distance + (0.1 * prefix_length as f32 * (1.0 - jaro_distance));

    if jaro_winkler_distance <= 1.0 {
        jaro_winkler_distance
    } else {
        1.0
    }
}

pub fn jaro_winkler(a: &str, b: &str) -> f32 {
    generic_jaro_winkler(&StringWrapper(a), &StringWrapper(b))
}
