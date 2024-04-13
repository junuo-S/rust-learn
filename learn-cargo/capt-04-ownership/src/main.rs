use std::ops::Index;

fn main()
{
    let mut s1 = String::from("hello");
    s1.push_str(", world");
    println!("s1 = {}", s1);

    let s2 = s1;
    // 上面那句话是浅拷贝，堆内存不会拷贝，未避免二次释放，所以rust将废弃掉s1. 可以认为是移动构造
    // 如果堆内存也需要拷贝，可以let s2 = s1.clone()
    //println!("s1 = {}", s1); // error: value borrowed here after move

    let str = String::from("hello world");
    let index = getWhiteCharIndex(&str);
    println!("index = {}", index);

    // 字符串切片
    let hello = &str[..5];
    let world = &str[6..];
    let whole = &str[..];
    println!("{},111 {}", hello, world);

    let ret = getFirstWord(&str);
    println!("ret = {}", ret);

    // 其它类型切片
    let array = [1, 2, 3, 4, 5];
    let slice = &array[1..3];
}

// 一个函数，传入一个字符串，返回里面的第一个空格的索引。如果无空格则返回字符串长度
fn getWhiteCharIndex(string: &String) -> usize
{
    let byteArray = string.as_bytes();
    for (i, &ch) in byteArray.iter().enumerate()
    {
        if ch == b' '
        {
            return i;
        }
    }
    return string.len();
}

fn getFirstWord(string: &String) -> &str
{
    let byteArray = string.as_bytes();
    for (i, &ch) in byteArray.iter().enumerate()
    {
        if ch == b' '
        {
            return &string[..i];
        }
    }
    return &string[..];
}
