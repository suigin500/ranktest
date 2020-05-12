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
pub fn average(mut ope: Vec<i32>) -> i32 {
    let mut average : i32 = 0;

    for i in &mut ope {
        average += *i;
    }
    average /= ope.len() as i32;

    return average;
}