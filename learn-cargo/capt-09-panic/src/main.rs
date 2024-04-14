fn testPanic()
{
    let vector = vec![1, 2, 3];
    // let number = vector[99];
}

use std::fs::File;
use std::hint::black_box;
use std::io;
use std::io::{ErrorKind, Read};

const fileName: &str = "D:/Honor/Desktop/rust.txt";

fn testResult()
{
    let result = File::open(fileName);
    let file = match result {
        Ok(file) => file,
        Err(error) => match error.kind()
        {
            ErrorKind::NotFound => match File::create(fileName)
            {
                Ok(fc) => fc,
                Err(e) => panic!("Error creating file: {:?}", e),
            },
            otherError=> panic!("Error opening file: {:?}", otherError),
        }
    };
}

fn broadcastResult() -> Result<String, io::Error>
{
    let mut f = File::open(fileName);
    let mut file = match f
    {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut str = String::new();
    let ret = file.read_to_string(&mut str);
    return match ret {
        Ok(content) => Ok(str),
        Err(e) => return Err(e),
    };
}

// ？ 放在Result后的作用：
//  1、如果Result是Ok：Ok中的值就是表达式的结果，然后继续执行程序
//  2、如果Result是Err：Err就是整个函数的返回值，相当于使用了return

fn broadcastResult2() -> Result<String, io::Error>
{
    let mut file = File::open(fileName)?;
    let mut str = String::new();
    let ret = file.read_to_string(&mut str)?;
    return Ok(str);
}

fn main()
{
    testResult();
    broadcastResult();
    broadcastResult2();
}
