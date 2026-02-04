fn check_guess(guess:i32, secret: i32) -> i32{
    if guess == secret{
        0
    } else if guess > secret {
        1
    }else {
        -1
    }
}

fn main (){
    let secret:i32 = 6;

    let mut attempts:i32 =0;

    let mut guess:i32 = 1;

    loop {
        attempts +=1;

        let result = check_guess(guess, secret);

        if result == 0{
            println!("Guess {guess}: Correct!");
            break;
        } else if result == 1 {
            println!("Guess {guess}: Too high!")
        } else {
            println!("Guess {guess}: Too low");
        }
        guess += 1;
    }
    println!("It took {attempts} guesses to find the secret humber.")
}