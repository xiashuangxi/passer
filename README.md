# Passer
[yuque]平台 API 的 Rust 实现。

> ⚠ 此库没有经历全面的测试，仅建议学习使用。后续会根据自身时间安排进行维护，但不会很勤。

## 功能列表
- ✅ 用户信息模块(User)
- ✅ 组织信息模块(Group)
- ✅ 知识库模块(Repo)
- ✅ 文档模块(Doc)

## 使用
在`Cargo.toml`中添加依赖。
```
[dependencies]
passer = { git = "https://github.com/xiashuangxi/passer.git" }
```

## 示例
``` rust
use passer;
fn main() {
    // OK
    // let user: passer::user::UserEntity = passer::client::Client::new("xxxxxxxx").get_user().get().unwrap();
    let user = passer::client::Client::new("xxxxxxxx").get_user().get().unwrap();
    println!("{:?}", user);
}
```

## 当前状态
此库当前还处于不稳定状态。

## 开源协议
[MIT] / [Apache 2.0]

[yuque]: https://www.yuque.com
[MIT]: https://github.com/xiashuangxi/passer/blob/master/LICENSE-MIT
[Apache 2.0]: https://github.com/xiashuangxi/passer/blob/master/LICENSE-APACHE