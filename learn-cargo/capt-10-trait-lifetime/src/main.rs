use std::fmt::{Debug, Display};

// 定义trait 类似纯虚基类
pub trait Summary
{
    // 也可以给一个默认实现
    fn summarize(&self) -> String;
    fn summarize2(&self) -> String
    {
        return "".to_string();
    }
}

pub struct NewsArticle
{
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle
{
    fn summarize(&self) -> String
    {
        return format!("{} by {} ({})", self.headline, self.author, self.location);
    }
}

pub struct Tweet
{
    pub username: String,
    pub content: String,
    pub reply: String,
    pub retweet: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        return format!("{}: {}", self.username, self.content);
    }
}

// trait 作为函数参数：可以接受所有实现了Summary的结构，适用于简单的情况
fn traitAsArgument(item: impl Summary)
{

}

// trait bound的方式，适用于复杂点的情况
fn traitAsArgument2<T: Summary>(item: T)
{

}

// 要求实现多个trait的写法
fn traitAsArgument3(item: impl Summary + Display)
{

}

fn traitAsArgument4<T: Summary + Display>(item: T)
{

}

// where子句
fn traitAsArgument5<T: Summary + Display, U: Clone + Debug>(a: T, b: U) {}
fn traitAsArgument6<T, U>(a: T, b: U) where T: Summary + Display, U: Clone + Debug {}

fn returnTrait() -> impl  Summary {}


fn main()
{

}
