use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("숫자를 맞춰 보세요!!");

    let secret_number: u32 = rand::thread_rng().gen_range(1, 101);

    println!("시크릿 숫자는: {}", secret_number);

    loop {
        println!("숫자를 입력해보세요");

        // mutable
        let mut guess  = String::new();

        // get warning message
        // io::stdin().read_line(&mut guess);
        
        io::stdin().read_line(&mut guess).expect("읽기에 실패했습니다..ㅠ");
        
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("에러 발생!!");
                continue;
            },
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!!"),
            Ordering::Equal => {
                println!("You win!!");
                break;
            },
        }
    }
}
