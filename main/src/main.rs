extern crate ranktest;
extern crate rankcorcoef;

use rankcorcoef::average;

fn main() {
    let data : Vec<i32> = vec![1,2,3,4,5,6];
    let ave : i32;

    ave = average(data);

    println!("{}",ave);
}
