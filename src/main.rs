use ::rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // console.log()みたいなもの
    println!("Guess the number!");

    // 変数宣言
    let secret_number = rand::thread_rng().gen_range(1..101);

    // loopキーワード: 無限ループを作り出す. break;で終了
    loop {
        println!("Please input your guess.");

        // 新しく空文字を生成する. new String()みたいなもの
        let mut guess = String::new();

        // ターミナル上で入力を受け付ける.
        // read_line()はResult型を返すので,expectメソッドが実行される.
        // expectメソッドはErrならクラッシュさせ、 Okそのまま返す.
        
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // trimで前後の空白を取り除いてparseする.
        // Okなら返す,Errならもう一度

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),  // 小さすぎ！
            Ordering::Greater => println!("Too big!"), // 大きすぎ！
            Ordering::Equal => {                        
                println!("You win!");                  // 君の勝ち！
                break;
            }
        }
    }
}
