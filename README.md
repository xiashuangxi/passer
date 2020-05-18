# Passer
[yuque]平台 API 的 Rust 实现。

> ⚠ 此库没有经历全面的测试，仅建议学习使用。后续会根据自身时间安排进行维护，但不会很勤。

## 功能列表
- ✅ 用户信息模块(User)
- ✅ 组织信息模块(Group)
- ✅ 知识库模块(Repo)
- ✅ 文档模块(Doc)

## 使用
在`Cargo.toml`中添加crate的依赖。
```
[dependencies]
passer = { git = "https://github.com/xiashuangxi/passer.git" }
```

## 示例
``` rust
use passer;

let user = passer::client::Client::new("xxxx").get_user().get().unwrap();
```

## 当前状态
此库当前还在孵化阶段，还存在很多错误和不稳定的问题。

[yuque]: https://www.yuque.com