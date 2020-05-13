# Passer
[yuque]平台 API 的 Rust 实现。

> ⚠ 此库仅建议学习使用。后续会根据自身时间安排进行维护，但不会很勤。

## 功能列表
- ✅ 用户信息模块(User)
- ⬜ 组织信息模块(Group)
- ⬜ 知识库模块(Repo)
- ⬜ 文档模块(Doc)

## 使用
``` rust
use passer;

let user = passer::client::Client::new("xxxx").get_user().get().unwrap();
```

## 当前状态
此库当前还在孵化阶段，还存在很多错误和不稳定的问题。

## License
MIT

[yuque]: https://www.yuque.com