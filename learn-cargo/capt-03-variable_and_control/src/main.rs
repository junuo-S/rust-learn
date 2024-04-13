fn main()
{
    // 创建元组 tuple. 元组声明之后长度不可变，里面数据的数据类型不必相同
    let tup: (i32, f64, &str) = (100, 3.14, "hello");
    // 使用结构匹配来解析元组数据
    let (x, y, z) = tup;
    println!("结构匹配解析元组数据：{}, {}, {}", x, y, z);
    println!("使用下标访问元组数据：{}, {}, {}", tup.0, tup.1, tup.2);

    // 创建数组，数组声明之后长度不可变，且里面的数据类型必须一致
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    // 下面一句相当于let array2 = [3, 3, 3, 3, 3]
    let array2 = [3; 5];
    // 数组元组的访问
    let number1 = array[0];
    let number2 = array[1];
    // 编译器会在编译的时候对索引做简单的检查，检查出数组越界编译时即可报错

    // 函数示例
    function01();
    function02(5, 3.14);
    function03();

    // if else
    let condition = false;
    // if else也是表达式，也可以返回值，但是不同分支返回的值必须是相同类型
    let a = if condition { 5 } else { 6 };

    // 循环
    let number = loop
    {
        // loop 循环是死循环，直道主动break
        println!("loop");
        break 1 // loop循环也是表达式，可以赋值给变量，break后面就是表达式的值
    };

    // while 循环就是一般的用法
    // for循环遍历数组
    let array = [1, 3, 4, 5];
    for value in array.iter()
    {
        println!("value = {}", value);
    }

    // Range用法：(1..5)表示生成[1,5)的数字，rev()是反转序列
    let range = (1..5).rev();
}

fn function01()
{
    println!("function01");
}

fn function02(x: i32, y: f64)
{
    println!("x = {}, y = {}", x, y);
}

fn function03()
{
    let x =
        {
            let y = 1;
            y + 1 // 作为这个表达式的返回值，注意：末位无分号
        };
    println!("function03 x = {}", x);
}

// ->指定返回值类型
fn function04() -> i32
{
    // 函数的返回值可以是函数的最后一个表达式
    // 也可以使用return 提前返回
    1 + 6
}
