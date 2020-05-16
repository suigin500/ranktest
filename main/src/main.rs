extern crate ranktest;
extern crate rankcorcoef;

use rankcorcoef::kendall_coefficient;           // ケンドールの順位相関係数

fn main() {
    let datax : Vec<f32> = vec![23.6,25.3,18.7,21.3,16.9,20.8];
    let datay : Vec<f32> = vec![118.0,111.0,116.0,128.0,109.0,121.0];
    let ave : f32;

    ave = kendall_coefficient(datax,datay);

    println!("{}",ave);
}
