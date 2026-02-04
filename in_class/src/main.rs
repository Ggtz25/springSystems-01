  //  let mut result : f32 = 0.0; // int
  //  let x:i32 = 5; //float
  //  result = result + x as f32; // no implicit conversion

  //  println!("{}",result);
// let x:i32 = 5;
   //x = 1.0.12; // you can't
    
    // let x:f32 = x as f32 + 1.012;
    // println!("{}",x)

// let x = 5;
 // {
 //   let x = x + 10;
 //   println!("x: {}", x );
 // } // free will be called for you 

 //   println!("x: {}", x);   
 

fn double(x:i32) -> i32{
  
  //return x*2;
  {
    let y = 10;
    x*2*y
  }

}
fn main() {

 println!("Double {} equals to {}",5, double(5)); 
       
}
