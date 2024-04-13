enum IpAddrKind
{
    IPV4,
    IPV6,
}

enum IpAddrKind2
{
    IPV4(u32, u32, u32, u32),
    IPV6(String),
}

fn main()
{
    let four = IpAddrKind::IPV4;
    let six = IpAddrKind::IPV6;
    
    let loopbackv4 = IpAddrKind2::IPV4(127, 0, 0, 1);
    let loopbackv6 = IpAddrKind2::IPV6(String::from("::1"));
    
    route(four);
    route(six);
    
    // std中的Option枚举
    let someNumber = Some(1);
    let someString = Some(String::from("hello"));
    let someNone: Option<i32> = None;
    // let ret = 1 + someNumber; // error: no implementation for `{integer} + Option<{integer}>`
    
    matchBindValue(&loopbackv6);
}

fn route(ipkind: IpAddrKind) {}

fn matchBindValue(ip: &IpAddrKind2)
{
    // match必须穷举所有枚举值，少了会报错
    // 或者使用_通配符，匹配所有剩余情况
    match ip
    {
        IpAddrKind2::IPV4(_, _, _, _) => {}
        IpAddrKind2::IPV6(addr) => 
            {
                // 获取到枚举中绑定的值
                println!("{}", addr);
            }
    }
    
    let a = 8u8;
    match a 
    {
        5 => println!("five"),
        _ => println!("else"), 
    }
    
    // 以上代码只关心a是5的情况，其它情况不关心，可以使用 if let
    if let 5 = a
    {
        println!("if let five");
    }
    else 
    {
        println!("if let else");
    }
}
