## 本仓库为作者本人学习rust过程记录

### 一、命令记录

```rust
rustup doc // 打开本地帮助文档
rustc main.rs // 编译rs文件，生成可执行文件
cargo new xxx // 使用cargo创建xxx项目，--vcs参数可以指定版本控制工具，默认是git。--vcs none表示不使用版本控制工具
cargo build // 编译cargo项目
cargo build --release // release编译
cargo check // 检查源代码是否可以通过编译，而非真的编译，所以速度比cargo build快很多
cargo run // 编译加运行。如果已编译过则直接运行
cargo update // 更改cargo.toml文件内的依赖包版本后并不会生效，会优先室友cargo.lock内的版本，此命令会重新生成cargo.lock
```
## 二、概念
* crate：rust的第三方包