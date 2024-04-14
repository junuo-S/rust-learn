mod frontOfHouse
{
    pub mod hosting
    {
        pub fn addToWaitList() {}
    }
}

pub fn eatAtRestaurant()
{
    // 绝对路径调用(相对独立，用的多)
    crate::frontOfHouse::hosting::addToWaitList();
    // 相对路径调用
    frontOfHouse::hosting::addToWaitList();
}

fn serveOrder() {}
mod backOfHouse
{
    fn fixInCorrectOrder()
    {
        cookOrder();
        // 取到模块的上一级的路径去访问
        super::serveOrder();
    }
    fn cookOrder() {}

    pub struct Breakfast
    {
        pub toast: String, // 公有字段
        seasonalFruit: String // 默认是私有字段
    }

    pub enum Appetizer
    {
        Soup, // 枚举值不需要加pub。默认就是公有
        Salad,
    }
}

// 一般use引入函数的时候，引入到它的父级
// 引入struct等的时候，直接引入具体的内容
use std::collections::HashMap;
use std::collections::HashMap as MyMap; // 可以使用as取别名

// 可以使用 pub use 让外部代码可以访问到use的模块

//use std::{io, cmp::Ordering};// 允许这样导入
//use std::io::{self, Write}; // 引入std::io  std::io::Write
// use ::std::collections::*; // 引入所有，谨慎使用

fn main()
{
    let mut map: HashMap<i32, String> = HashMap::new();
    let mut map2: MyMap<i32, String> = MyMap::new();
    map.insert(1, String::from("zhangsan"));
    map2.insert(2, String::from("lisi"));
}
