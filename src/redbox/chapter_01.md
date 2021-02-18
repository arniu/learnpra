# 第 1 章：建立服务器

## 准备工作

安装好 Rust 开发工具，及相关工具：

1. 代码编辑器
1. Redis，用于测试服务器
1. cargo-edit，用于添加包

## 正式开始

新建项目，然后添加包：

```
cargo add tokio --features full
```

新建 `server.rs` 文件，内容如下：

```rust
use std::io;

pub async fn run(port: u16) -> io::Result<()> {
    todo!()
}
```

修改 `main.rs` 文件如下：

```rust
mod server;

#[tokio::main]
async fn main() {
    let port = 6379_0; // Redis port is 6379
    if let Err(err) = server::run(port).await {
        eprintln!("Failed with: {}", err);
    }
}
```

## 建立连接
