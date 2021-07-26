fn get_non_empty<T>(list: &[T]) -> Option<&[T]> {
    use std::ops::Not;
    list.is_empty().not().then(||list)
}

fn is_odd(n: usize) -> bool { n % 2 == 1 }

// pub fn sum(ints: &[i32]) -> i32 {
//     ints.iter().fold(0, |a, b| a + b)
// }

// pub fn mean(ints: &[i32]) -> Option<f64> {
//     let ints = get_non_empty(ints)?;
//     let mean = |ints: &[i32]| {
//         sum(ints) as f64 / ints.len() as f64
//     };
//     get_non_empty(ints).map(mean)
// }
pub fn mean(ints: &[i32]) -> Option<f64> {
    let sum: i32 = ints.iter().sum();
    let len = get_non_empty(ints)?.len();
    Some( sum as f64 / len as f64 )
}


fn get_sorted_list(ints: &[i32]) -> Vec<i32> {
    let mut l = vec![0; ints.len()];
    l.clone_from_slice(ints);
    l.sort();
    l
}

pub fn median(ints: &[i32]) -> Option<f64> {
    let sorted = get_non_empty(ints).map(get_sorted_list)?;
    let len = sorted.len();
    let ret = if is_odd(len) {
        sorted[(len - 1) / 2] as f64
    } else {
        let m = len / 2;
        let a = sorted[m-1];
        let b = sorted[m];
        (a + b) as f64 / 2.
        // ((a + b) / 2) as f64 // ❌
    };
    Some(ret)
}

fn find_max<I>(iter: I) -> Option<I::Item>
    where I: Iterator,
          I::Item: Ord,
{
    iter.reduce(|a, b| {
        std::cmp::max(a, b)
    })
}

use std::collections::HashSet;
use std::collections::HashMap;
pub fn mode(ints: &[i32]) -> HashSet<i32> {
    let mut map = HashMap::new();
    ints.iter().for_each(|int| {
        let value_ref = map.entry(*int).or_insert(0);
        *value_ref += 1;
    });
    find_max(map.values()).map(|max|{
        map.iter().fold(HashSet::new(), |mut ret, (k, v)| {
            if v == max { ret.insert(*k); }
            ret
        })
    }).unwrap_or(HashSet::new())
}


#[cfg(test)]
mod tests {
    #[test]
    fn mean() {
        use super::*;
        assert_eq!(mean(&[]), None);
        assert_eq!(mean(&[1]), Some(1.));
        assert_eq!(mean(&[3, 4]), Some(3.5));
        assert_eq!(mean(&[2, 1, 3]), Some(2.));
    }
    #[test]
    fn calculates_median_of_list_with_odd_length() {
        use super::*;
        let numbers = vec![1, 2, 5, 6, -1];
        assert_eq!(median(&numbers), Some(2.));
    }


    #[test]
    fn median() {
        use super::*;
        // https://en.wikipedia.org/wiki/Median
        assert_eq!(median(&[]), None);
        assert_eq!(median(&[1]), Some(1.0));
        assert_eq!(median(&[2, 1]), Some(1.5));
        assert_eq!(median(&[2, 1, 3]), Some(2.0));
        assert_eq!(median(&[1, 3, 3, 6, 7, 8, 9]), Some(6.0));
        assert_eq!(median(&[1, 2, 3, 4, 5, 6, 8, 9]), Some(4.5));
    }

    #[test]
    fn mode() {
        use super::*;
        fn to_hash_set(s:&[i32]) -> HashSet<i32> {
            s.iter().cloned().collect()
        }
        let a: HashSet<_> = to_hash_set(&[1]);
        let b: HashSet<_> = to_hash_set(&[1, 2]);
        let c: HashSet<_> = to_hash_set(&[4]);
        assert_eq!(mode(&[]), HashSet::new());
        assert_eq!(mode(&[1]), a);
        assert_eq!(mode(&[1, 2, 3, 1]), a);
        assert_eq!(mode(&[1, 2, 2, 3, 1]), b);
        assert_eq!(mode(&[1, 2, 2, 3, 1, 4, 4, 4]), c);
    }
}
