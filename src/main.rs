#![allow(unused)]
use assert_approx_eq::assert_approx_eq;
use rand::Rng;
use std::time::{Duration, Instant};

fn main() {
    println!("Hello, world!");

    let mut rng = rand::thread_rng();
    let mut list = Vec::new();

    for i in 0..50000 {
        let n: i32 = rng.gen_range(-5000, 5000);
        list.push(n);
    }

    // println!("list: {:?}", list);

    // println!("starting naive method...");
    // let start = Instant::now();

    // let (avg, (a, b)) = min_abaverage_naive(&list);
    // println!("min ab average: ");
    // println!("avg: {}", avg);
    // println!("range indices: ({}, {})", a, b);
    // println!("range values: {:?}", &list[a..b+1]);

    // let duration = start.elapsed();
    // println!("finished naive method in {:?}", duration);

    println!("\n\nstarting smart method...");
    let start = Instant::now();
    
    let (avg, (a, b)) = min_abaverage_smart(&list);
    println!("min ab average: ");
    println!("avg: {}", avg);
    println!("range indices: ({}, {})", a, b);
    println!("range values: {:?}", &list[a..b+1]);
    
    let duration = start.elapsed();
    println!("finished smart method in {:?}", duration);
}


fn compute_average(list: &[i32], a: usize, b: usize) -> f32 {
    let count = (b - a + 1) as f32;
    let mut avg: f32 = 0.0;
    for i in a..b+1 {
        avg += (list[i] as f32) / count;
    }
    avg
}

/// Naive method: compute average for all possible (a,b) ranges
fn min_abaverage_naive(list: &[i32]) -> (f32, (usize, usize)) {
    let mut lowest = 1_000_000.0;
    let mut range: (usize, usize) = (0, 0);

    for i in 0..list.len() {
        for j in i+1..list.len() {
            let avg = compute_average(list, i, j);
            if avg < lowest {
                lowest = avg;
                range = (i, j);
            }
        }
    }
    (lowest, range)
}

/// Struct for holding information about the current range.
#[derive(Debug, Clone)]
struct Range<'a> {
    list: &'a [i32], // reference to full list
    left: usize, // left index of range
    right: usize, // right index of range
    sum: i32, // sum of values in range (kept to speed up recalculating averages)
    avg: f32, // average of this range
}

impl<'a> Range<'a> {
    /// Create a new Range
    fn new(list: &'a [i32], left: usize, right: usize) -> Range {
        let mut sum = 0;
        for i in left..right + 1 {
            sum += list[i];
        }
        let mut range = Range { list, left, right, sum, avg: 0.0 };
        range.update_avg();
        range
    }

    /// Convenience method for hiding details of calculating the average.
    fn update_avg(&mut self) {
        let length = self.right - self.left + 1;
        self.avg = (self.sum as f32) / (length as f32);
    }

    /// Extend the range into the next element of the list.
    fn extend(&mut self) {
        self.right += 1;
        self.sum += self.list[self.right];
        self.update_avg();
    }

    /// Transform the range into the pair (current last element, next element).
    fn new_pair(&mut self) {
        self.left = self.right;
        self.right = self.right + 1;
        self.sum = self.list[self.left] + self.list[self.right];
        self.update_avg();
    }

    /// Check the next element in the list to see what should be done.
    fn peek(&self) -> Peek {
        if self.right == self.list.len() - 1 {
            Peek::EndOfList
        } else {
            let next = self.list[self.right + 1];
            let curr_len = self.right - self.left + 1;
            let ext_avg = (self.sum + next) as f32 / (curr_len + 1) as f32;
            let pair_avg = (self.list[self.right] + next) as f32 / 2.0;

            if pair_avg < self.avg && pair_avg < ext_avg {
                Peek::NewPair
            } else if ext_avg < self.avg && ext_avg < pair_avg {
                Peek::ExtendRange
            } else if self.right + 1 == self.list.len() - 1 {
                Peek::EndOfList
            } else {
                Peek::Nothing
            }
        }
    }
}

enum Peek{
    ExtendRange,
    NewPair,
    Nothing,
    EndOfList
}

/// Traverse the list from left to right, keeping track of the optimal range
/// of the elements seen so far.
/// For each next element, either
///   - add it to the current range
///   - make a new range of size 2 consisting of the new element and the one before
///   - store the current range, and continue finding the optimal range for the remaining part of the list
///
/// Returns (average, (left index, right index))
fn min_abaverage_smart(list: &[i32]) -> (f32, (usize, usize)) {
    // Working range, updated while traversing the list.
    let mut range = Range::new(list, 0, 1);

    // Best range found so far.
    let mut best_range = range.clone();

    loop {
        match range.peek() {
            Peek::ExtendRange => {
                // Add next element to current range.
                range.extend();
            }
            Peek::NewPair => {
                // Transform range into pair (current last elt, new elt)
                range.new_pair();
            }
            Peek::Nothing => {
                // Cannot use next element. Update best range if applicable,
                // and make a new one starting from here.
                if range.avg < best_range.avg {
                    best_range = range.clone();
                }
                range = Range::new(list, range.right, range.right + 1);
            }
            Peek::EndOfList => {
                // Reached end of the list. Update best range if applicable.
                if range.avg < best_range.avg {
                    best_range = range.clone();
                }
                break;
            }
        }
    }

    // Return values of best range.
    (best_range.avg, (best_range.left, best_range.right))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let list = vec![0, 1];
        let (avg, range) = min_abaverage_naive(&list);
        assert_eq!(avg, 0.5);
        assert_eq!(range, (0, 1));
    }

    #[test]
    fn test2() {
        let list = vec![5, 7, 4, 8, 1];
        let (avg, range) = min_abaverage_naive(&list);
        assert_eq!((avg, range), (4.3333335, (2, 4)));
    }

    #[test]
    fn test3() {
        let list = vec![6, 7, 0, 9, 3, 2];
        let (avg, range) = min_abaverage_naive(&list);
        assert_eq!((avg, range), (2.5, (4, 5)));
    }

    #[test]
    fn test4() {
        let list = vec![4, 8, -2, 5, 1, 2, 3, 4, 5];
        let (avg, range) = min_abaverage_naive(&list);
        assert_eq!((avg, range), (1.3333333, (2, 4)));
    }


    #[test]
    fn test1_smart() {
        let list = vec![0, 1];
        let (avg, range) = min_abaverage_smart(&list);
        assert_eq!(avg, 0.5);
        assert_eq!(range, (0, 1));
    }

    #[test]
    fn test2_smart() {
        let list = vec![5, 7, 4, 8, 1];
        let (avg, range) = min_abaverage_smart(&list);
        assert_eq!((avg, range), (4.3333335, (2, 4)));
    }

    #[test]
    fn test3_smart() {
        let list = vec![6, 7, 0, 9, 3, 2];
        let (avg, range) = min_abaverage_smart(&list);
        assert_eq!((avg, range), (2.5, (4, 5)));
    }

    #[test]
    fn test4_smart() {
        let list = vec![4, 8, -2, 5, 1, 2, 3, 4, 5];
        let (avg, range) = min_abaverage_smart(&list);
        assert_eq!(range, (2, 4), "range incorrect");
        assert_approx_eq!(avg, 1.3333333, 0.000001);
    }

}
