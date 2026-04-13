// Higher-Ranked Trait Bounds: Use HRTBs for closure traits
fn iterate_and_modify<F>(data: &mut [i32], func: F) 
where F: for<'a> Fn(&'a mut i32)
{
    for item in data {
        func(item);
    }
}

fn main() {
    let mut numbers = [1,2,3,4,5,67,7];
    iterate_and_modify(&mut numbers, |num| *num += 1);
    println!("{:?}", numbers);
}
