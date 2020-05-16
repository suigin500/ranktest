#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
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
pub fn kendall_coefficient(mut x: Vec<f32>, mut y: Vec<f32>) -> f32 {
    // ケンドールの順位相関係数
    let mut coefficient : f32;

    // ２つの順位データの個数が異なっているならエラー終了
    assert_eq!( x.len(), y.len() );
    if x.len() != y.len() {
        return 0.0
    }

    // xとyのベクターの要素を昇順に直す
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

    // y[i] < y[j] (i<j) となるyの個数を数える
    let mut supcount : i32 = 0 ;
    for i in 0..x.len() {
        for j in (i+1)..x.len() {
            if y[i] < y[j] {
                supcount += 1;
            }
        }
    }

    // 相関係数の演算
    coefficient = 4.0;
    coefficient *= supcount as f32;
    coefficient /=  x.len() as f32 ;
    coefficient /= ( x.len() as f32 - 1.0 ) as f32;
    coefficient -= 1.0;


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