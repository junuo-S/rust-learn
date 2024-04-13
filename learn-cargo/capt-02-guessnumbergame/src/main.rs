use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main()
{
    println!("this a guess number game");

    let a = 1;
    let b = a;
    // a = 2; // error: cannot assign twice to immutable variable. rust使用let声明的变量默认是immutable 不可变的

    // 生成个随机数
    let secretNumber = rand::thread_rng().gen_range(1..101);
    //println!("random number = {}", secretNumber);

    loop
    {
        println!("please guess a number: ");
        let mut number = String::new();
        // io::stdin().read_line返回的是io::Result. Ok代表成功，Err代表失败，还会附带错误信息
        io::stdin().read_line(&mut number).expect("read failed");
        //println!("input number = {}", number);

        let number: u32 = match number.trim().parse()
        {
            Ok(num) => num,
            Err(_) => continue,
        };
        match number.cmp(&secretNumber)
        {
            Ordering::Less => println!("Too small"),
            Ordering::Equal =>
            {
                println!("You win");
                break;
            },
            Ordering::Greater => println!("Too Big"),
        }
    }
}
