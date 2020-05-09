# Passer
[yuque](https://www.yuque.com/)平台 API 的 Rust 实现。

> ⚠ 这个库目前还未稳定，仅建议学习使用。后续会根据自身时间安排进行维护，但不会很勤。

# 使用
``` rust
use passer;

let user = passer::client::Client::new("xxxx").get_user().get().unwrap();
```

# License
MIT