use std::cmp::Ord;
use std::thread;
use std::time::Duration;

const N: usize = 5;

fn compare_ordered_pairs<T>(left: &[T], right: &[T]) -> i8  where T: Ord {
    assert!(left.len() == right.len(), "slices must be of equal length");
    for i in 0..left.len() {
        if left[i] > right[i] {return 1};
        if left[i] < right[i] {return -1};
    }
    return 0
}

fn l1(i: usize, choosing: & mut [i32], number: & mut [i32]) {
    loop {
        choosing[i] = 1;
        number[i] = 1 + number[i..N].iter().max().unwrap();
        choosing[i] = 0;
        for j in 0..N {
            while choosing[j as usize] != 0 {
            }
            while number[j] != 0 && compare_ordered_pairs(&[number[j], j as i32], &[number[i], i as i32]) < 0  {
            }
        }
        println!("processor #{} begin critical section", i);
        number[i] = 0;
        // TODO: do some stuff here
        println!("processor #{} end critical section", i);
    }
}

fn bakery() {
    let mut choosing: [i32; N] = [0; N];
    let mut number: [i32; N] = [0; N];

    // TODO: start separate threads for each i
    for i in 0..N {
        thread::spawn(|| {
            l1(i, & mut choosing, & mut number);
        });
    }
}

fn main() {
    bakery()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn try_less_than_ordered_pairs_test() {
        assert_eq!(compare_ordered_pairs(&[0, 0], &[0,1]), -1);
        assert_eq!(compare_ordered_pairs(&[1, 0], &[0,1]), 1);
        assert_eq!(compare_ordered_pairs(&[0, 0], &[0,0]), 0);
    }
}