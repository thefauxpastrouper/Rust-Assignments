// Zero-Cost Abstractions: Identify and use zero-cost abstractions

fn sum_squares_iter(v: &mut [i32]) -> i32 {
    v.iter().map(|x| x*x).sum()
}

fn sum_squares_loop(v: &mut [i32]) -> i32 {
    let mut sum = 0;
    for i in 0..v.len() {
        sum += v[i]*v[i];
    }
    sum
}

fn main() {
    let mut v = [1,2,3,4];
    let sum1 = sum_squares_iter(&mut v);
    let sum2 = sum_squares_loop(&mut v);
    
    println!("Hello, world! Results: {}, {}", sum1, sum2);
}
