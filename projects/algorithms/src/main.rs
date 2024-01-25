use std::mem::swap;

fn main() {
    let m:i64 = 100000000;
    for i in 1..=m{
        let _m = i;
    }
    print!("I am done");
}

fn gcd( m:&mut i32, n:&mut i32) -> i32{
    if m < n {
        swap(m, n)
    }
    if *m % *n == 0 {
        return *n;
    }else {
        let mut r = *m % *n;
       return gcd(n,&mut r );
    }
}

fn gcd_while(m:&mut i32, n:&mut i32) -> i32 {
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

#[cfg(test)]
mod test {
    use crate::{gcd, gcd_while};

    #[test]
    fn gcd_of_101_and_2() {
        let mut m = 101;
        let mut n = 2;
        let result = gcd(&mut m, &mut n);
        let result_while = gcd_while(&mut m, &mut n);
        assert_eq!(1,result);
        assert_eq!(1,result_while);
    }
    #[test]
    fn gcd_of_100_and_2() {
        let mut m = 100;
        let mut n = 2;
        let result = gcd(&mut m, &mut n);
        assert_eq!(2,result);
    }
}
