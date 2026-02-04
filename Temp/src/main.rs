const FREEZING_POINT_F: f64=32.0;

fn f_to_c(f: f64) ->f64{
    (f-32.0)* 5.0/9.0
}
fn c_to_f(c: f64) -> f64{
    (c * 9.0/5.0) + 32.0
}

fn main(){
    let mut temp_f: f64 = FREEZING_POINT_F;

    let temp_c= f_to_c(temp_f);
    println!("{temp_f}F = {:.2}C", temp_c);

    for _ in 0..5{ 
        temp_f += 1.0;
        let temp_c = f_to_c(temp_f);
        println!("{temp_f}F = {:.2}C", temp_c);
    }
}