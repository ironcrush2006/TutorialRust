use std::io;
use rand::Rng;


fn call(){ 
    println!("Guess the number");
    let  number=rand::thread_rng().gen_range(1..20);
    println!("The number to guess is {}",number);

loop{
    println!("Please input your guess");

    let mut guess =String::new();

    io::stdin().read_line(&mut guess).expect("Failed to get the guess");

    println!("You guessed :{}",guess);
     
      let guess:u32=match  guess.trim().parse()
      { 
        Ok(num)=>num,
        Err(_)=>continue,
      };

    if guess<number { 
        println!("guess is less");
    }
    else if guess>number{ 
          println!("guess is high");
    }
    else { 
        println!("guess is correct");
        break;
    }
}
}
fn main(){ 
   call();
}