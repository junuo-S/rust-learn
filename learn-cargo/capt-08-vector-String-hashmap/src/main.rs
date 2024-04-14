use std::collections::HashMap;
use std::ffi::CString;

fn testVector()
{
    let mut vector: Vec<i32> = Vec::new();
    let mut vector2 = vec![1, 2, 3];
    vector.push(22); // 放入元素
    let mut number = vector[0];
    let number2: &i32 = &vector[0];
    match vector.get(0)
    {
        Some(number) => println!("exist: {}", number),
        None => println!("not exist"),
    }

    for num in &vector2
    {
        println!("{}", num);
    }
}

enum SpreadSheetCell
{
    Int(i32),
    Float(f64),
    Text(String),
}
fn vectorCase()
{
    // 使用可带数据的枚举，来让vector里存放不同类型的数据
    let row = vec![
        SpreadSheetCell::Int(32),
        SpreadSheetCell::Float(3.14),
        SpreadSheetCell::Text(String::from("123")),
    ];
}

fn testString()
{
    let mut emptyStr = String::new();
    let mut string1 = "hello world".to_string();
    let mut string2 = String::from("abc");
    emptyStr.push_str("abc"); // 附加字符串切片
    emptyStr.push('c'); // 附加单个字符
    let ret = string1 + &string2; // 拼接字符串, 取得string1的所有权，此后string1已不可用
    let ret2 = format!("{}-{}-{}", emptyStr, emptyStr, string2); // 不会取得任何参数的所有权，此后都可以使用

    // let ch = emptyStr[0]; // 不可以使用索引访问 String其实是Vec<u8>
    let englishString = "Hello World".to_string(); // len=11，一个字符占一个字节
    let zhcnString = "张三".to_string(); // len = 6, 一个字符占3个字节。所以不可通过索引来访问字符
    println!("englishString len = {}, zhcnString len = {}", englishString.len(), zhcnString.len());

}

fn testHashMap()
{
    let mut map = HashMap::new();
    map.insert("zhangsan".to_string(), 10);
    map.insert("lisi".to_string(), 50);

    let vector = vec!["blue".to_string(), "yellow".to_string()];
    let vector2 = vec![20, 30];
    let map2: HashMap<_, _> = vector.iter().zip(vector2.iter()).collect();

    match map.get("zhangsan")
    {
        Some(age) => println!("{}", age),
        None => println!("not exist"),
    }

    // 遍历hashmap
    for (key, value) in &map
    {
        println!("key = {}, value = {}", key, value);
    }
    // 判断key如果存在就不插入，不存在则插入
    map.entry("zhangsan".to_string()).or_insert(100);

}

fn main()
{
    testVector();
    vectorCase();
    testString();
    testHashMap();
}
