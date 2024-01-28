use algorithms::*;
use rand::Rng;
use std::time::Instant;

#[test]
fn gcd_of_101_and_2() {
    let mut m = 101;
    let mut n = 2;
    let result = gcd(&mut m, &mut n);
    let result_while = gcd_while(&mut m, &mut n);
    assert_eq!(1, result);
    assert_eq!(1, result_while);
}
#[test]
fn gcd_of_100_and_2() {
    let mut m = 100;
    let mut n = 2;
    let result = gcd(&mut m, &mut n);
    assert_eq!(2, result);
}
#[test]
fn selection_sort_arrray_elems() {
    let mut a = [4, 9, 1, 3, 10, 8];
    assert_eq!(selection_sort(&mut a.to_vec()), a.sort());
}
#[test]
fn selection_sort_vec_elems() {
    let mut a = vec![4, 9, 1, 3, 8, 0, 8];
    assert_eq!(selection_sort(&mut a), a.sort());
}
#[test]
fn selection_sort_10_000_random_integers() {
    let start_time = Instant::now();
    let mut random_elem: Vec<i32> = (0..10_000)
        .map(|_| rand::thread_rng().gen_range(-100..=100))
        .collect();
    assert_eq!(selection_sort(&mut random_elem), random_elem.clone().sort());
    println!("Elapsed time: {:?}", start_time.elapsed());
}
#[test]
fn insertion_sort_arrray_elems() {
    let mut a = [4, 9, 1, 3, 10, 8];
    assert_eq!(insertion_sort(&mut a.to_vec()), a.sort());
}
#[test]
fn insertion_sort_vec_elems() {
    let mut a = (0..10)
        .map(|_| rand::thread_rng().gen_range(-100..=100))
        .collect();
    assert_eq!(insertion_sort(&mut a), a.sort());
}
#[test]
fn insertion_sort_10_000_random_integers() {
    let start_time = Instant::now();
    let mut random_elem: Vec<i32> = (0..10_000)
        .map(|_| rand::thread_rng().gen_range(-100..=100))
        .collect();
    assert_eq!(insertion_sort(&mut random_elem), random_elem.clone().sort());
    println!("Elapsed time: {:?}", start_time.elapsed());
}
