use core::mem::swap;
use num_traits::Pow;



pub fn rust_speed(exponent: u8) {
    let m = Pow::pow(10u32, exponent);
    println!("{m}");
    for i in 1..=m {
        let _m = i;
    }
    print!("I am done");
}

pub fn selection_sort(unsorted_list: &mut Vec<i32>) {
    for i in 0..unsorted_list.len() {
        for j in i..unsorted_list.len() {
            if unsorted_list[i] > unsorted_list[j] {
                unsorted_list.swap(i, j);
            }
        }
    }
    print!("{:?}", unsorted_list)
}

pub fn insertion_sort(unsorted_list: &mut Vec<i32>) {
    for i in 0..unsorted_list.len() {
        for j in 0..i {
            if unsorted_list[j] > unsorted_list[i] {
                unsorted_list.swap(i, j);
            }
        }
    }
}

pub fn gcd(m: &mut i32, n: &mut i32) -> i32 {
    if m < n {
        swap(m, n)
    }
    if *m % *n == 0 {
        return *n;
    } else {
        let mut r = *m % *n;
        return gcd(n, &mut r);
    }
}

pub fn gcd_while(m: &mut i32, n: &mut i32) -> i32 {
    if m < n {
        swap(m, n)
    }
    while *m % *n != 0 {
        let rem = *m % *n;
        *m = *n;
        *n = rem;
    }
    return *n;
}