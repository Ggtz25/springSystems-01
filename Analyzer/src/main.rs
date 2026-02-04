fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn main(){
    let numbers = [3, 10, 15, 17, 4, 5, 2, 6, 13, 20];

    for n in numbers {
        if n % 3 == 0 && n % 5 == 0 {
            println!("{n}: FizzBuzz");
        }   else if n % 3 == 0 {
            println!("{n}: Fizz");
        }   else if n % 5 == 0 {
            println!("{n}: Buzz");
        } else {
            if is_even(n) {
                println!("{n}: Even");
            } else {
                println!("{n}: Odd");
            }
        }
    }
    let mut index = 0;
    let mut sum = 0;

    while index < numbers.len(){
        sum += numbers[index];
        index += 1;
    }
    println!("Sum of all numbers: {sum}");

    let mut largest = numbers[0];
    let mut i = 1;
    loop {
        if i >= numbers.len() {
            break;
        }
        if numbers[i] > largest {
            largest = numbers[i];
        }
        i += 1;
    }
    println!("Largest number: {largest}");
}