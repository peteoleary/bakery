

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


static mut choosing: [i32; N] = [0; N];
static mut number: [i32; N] = [0; N];

static mut critical_section_counter: [i32; N] = [0; N];

unsafe fn l1(i: usize) {
    for _ in 0..100 {
            choosing[i] = 1;
            number[i] = 1 + number.iter().max().unwrap();
            println!("processor #{} number is {}", i, number[i]);
            choosing[i] = 0;
            for j in 0..N {
                while choosing[j as usize] != 0 {
                    println!("processor #{} waiting for choosing #{}", i, j);
                    thread::sleep(Duration::from_millis(10));
                }
                let mut counter = 0;
                while number[j] != 0 && compare_ordered_pairs(&[number[j], j as i32], &[number[i], i as i32]) < 0  {
                    println!("processor #{} waiting for number {}", i, counter);
                    counter += 1;
                    thread::sleep(Duration::from_millis(10));
                }
            }
            println!("> processor #{} begin critical section", i);
            
            number[i] = 0;
            
            // TODO: do something interesting here, for now we will keep track of how many times each thread got the critical section
            critical_section_counter[i] += 1;
            
            println!("> processor #{} end critical section", i);
    }
}

fn bakery() {

    let mut children = vec![];
    for i in 0..N {
        children.push(thread::spawn(move || unsafe {
            l1(i);
        }));
    }

    for child in children {
        // Wait for the thread to finish. Returns a result.
        let _ = child.join();
    }

    for i in 0..N {
        unsafe {
            println!("processor #{} got the critical section {} times", i, critical_section_counter[i]);
        }
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