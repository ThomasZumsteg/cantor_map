use std::mem::swap;
use std::collections::HashSet;
use cantor::Point;

fn gcd(mut a: usize, mut b: usize) -> usize {
    if a < b {
        swap(&mut a, &mut b);
    }
    while b > 0 {
        a = a % b;
        swap(&mut a, &mut b);
    }
    a
}

fn gcd_method(size: usize) -> HashSet<Point> {
    let mut result = HashSet::new();
    for m in 0..size {
        for n in 0..size {
            if gcd(m, n) != 1 {
                result.insert(Point::new(m as i32, n as i32));                
            }
        }
    }
    result
}

fn main() {
    let points = gcd_method(100);
    println!("{:?}", points)
}
