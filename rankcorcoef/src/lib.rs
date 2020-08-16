#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    // ケンドールの順位相関係数のテスト
    #[test]
    fn kendall_coefficient_test(){
        let datax : Vec<f32> = vec![23.6,25.3,18.7,21.3,16.9,20.8];
        let datay : Vec<f32> = vec![118.0,111.0,116.0,128.0,109.0,121.0];
        let coefficient : f32;

        coefficient = kendall_coefficient(datax,datay);
        assert_eq!( coefficient, 0.2 , "coefficientの値({})と一致しない",  coefficient );
    }
}

#[no_mangle]
pub extern "stdcall" fn Spearman(count: i32) -> i32 {
    if count < 0 {
       println!("A negative count is not supported.");
        return 0;
    }

    return 1;
}

#[no_mangle]
// ケンドールの順位相関係数
// x : 順位データ1
// y : 順位データ2
pub extern "stdcall" fn kendall_coefficient(mut x: Vec<f32>, mut y: Vec<f32>) -> f32 {
    // ケンドールの順位相関係数
    let mut coefficient : f32;

    // ２つの順位データの個数が異なっているならエラー終了
     if x.len() != y.len() {
        panic!("２つの順位データの個数が異なっている");
    }

    // xとyのベクターの要素を昇順に並べ直す
    // バブルソートする
    let mut work : f32;
    for i in 0..x.len() {
        for j in 0..x.len() {
            if x[i] < x[j] {
                work = x[i];
                x[i] = x[j];
                x[j] = work;

                work = y[i];
                y[i] = y[j];
                y[j] = work;
            }
        }
    }

    // y[i] < y[j] (i<j) となるy[j]の個数を数える
    let mut supcount : i32 = 0 ;
    for i in 0..x.len() {
        for j in (i+1)..x.len() {
            if y[i] < y[j] {
                supcount += 1;
            }
        }
    }

    // 相関係数の演算
    let mut _work : f32;
    coefficient = 4.0;
    coefficient *= supcount as f32;
    _work =  x.len() as f32 ;
    _work *= ( x.len() as f32 - 1.0 ) as f32;
    coefficient -= _work;
    coefficient /= _work;

    return coefficient;
}

#[no_mangle]
pub fn average(mut ope: Vec<f32>) -> f32 {
    let mut average : f32 = 0.0;

    for i in &mut ope {
        average += *i;
    }
    average /= ope.len() as f32;

    return average;
}