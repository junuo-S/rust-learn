struct User
{
    username: String,
    email: String,
    age: u32,
    active: bool
}

// 可以定义tuple struct
struct Color (u32, u32, u32);
struct Point (u32, u32, u32);

#[derive(Debug)]
struct Rectangle
{
    width: u32,
    height: u32,
}

impl Rectangle
{
    fn area(&self) -> u32
    {
        return self.width * self.height;
    }

    fn isLargerThan(&self, other: &Rectangle) -> bool
    {
        return self.width > other.width && self.height > other.height;
    }

    // 关联函数
    fn square(size: u32) -> Rectangle
    {
        return Rectangle {width: size, height: size};
    }
}

fn createUser(username: String, email: String) -> User
{
    return User
    {
        username, // 可以这样简写，仅限同名，同类型
        email,
        age: 31,
        active: false,
    };
}

fn main()
{
    let user1 = User
    {
        username: String::from("sxw"),
        email: String::from("1@add"),
        age: 32,
        active: true,
    };

    let user2 = User
    {
        username: String::from("add"),
        ..user1 // 剩余部分使用user1
    };

    let color = Color(0, 0, 0);
    let point = Point(0, 0, 0);

    let r1 = (30, 50);
    println!("area01: {}", area01(r1));

    let r2 = Rectangle {width: 30, height: 50};
    println!("area02: {}", area02(&r2));

    println!("Rectangle.area: {}", r2.area());

    let rect1 = Rectangle{width: 30, height: 50};
    let rect2 = Rectangle{width: 10, height: 40};
    let rect3 = Rectangle{width: 10, height: 55};
    println!("rect1.isLargerThan(&rect2): {}", rect1.isLargerThan(&rect2));
    println!("rect1.isLargerThan(&rect3): {}", rect1.isLargerThan(&rect3));

    println!("{:?}", r2);
    println!("{:#?}", r2);

    let square = Rectangle::square(20);
}

fn area01(rect: (u32, u32)) -> u32
{
    return rect.0 * rect.1;
}

fn area02(rect: &Rectangle) -> u32
{
    return rect.width * rect.height;
}
