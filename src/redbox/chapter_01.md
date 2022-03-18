# 第 1 章：新建 Echo 服务器

## 准备工作

根据 `<https://rustup.rs/>` 指示，准备好 Rust 开发环境。

安装 `cargo-edit`，顺便检验 Rust 是否就绪。安装方法如下：

```shell
cargo install cargo-edit
```

> `cargo-edit` 是 `cargo` 的有力补充，它提供了 `add`、`rm` 和 `upgrade` 等命令，可以方便地管理项目依赖。

此外，准备好如下工具：

1. 代码编辑器，比如 Visual Studio Code；
2. Redis，用于测试服务器。

## 新建项目

新建项目，并切换到项目目录下：

```shell
cargo new --lib redbox && cd redbox
```

建立如下文件结构：

<pre>
.
├── Cargo.lock
├── Cargo.toml
├── LICENSE-APACHE
├── LICENSE-MIT
├── README.md
└── src
    ├── bin
    │   ├── redbox-cli.rs       # 客户端
    │   └── redbox-server.rs    # 服务器
    ├── lib.rs
    └── server.rs
</pre>

- 编辑 `redbox-cli.rs`

```rust
fn main() {
    println!("TODO: redbox-cli");
}
```

对应 Redis 客户端，本章暂不讨论。

- 编辑 `redbox-server.rs`

```rust
fn main() {
    println!("TODO: redbox-server");
}
```

对应 Redis 服务器，本章程序入口。

- 小测验

运行如下命令，看看各自输出了什么：

1. `cargo run`
1. `cargo run --bin redbox-cli`
1. `cargo run --bin redbox-server`

> 为方便起见，我们设置默认运行的程序，或别名：
>
> - 设置默认运行的程序
>
>   打开 `Cargo.toml` 设置 `[package.default-run]`：
>
>   ```toml
>   [package]
>   default-run = "redbox-server"
>   ```
>
>   这样运行 `cargo run` 即可启动服务器。
>
> - 设置别名
>
>   新建文件 `.cargo/config.toml` 设置别名：
>
>   ```toml
>   [alias]
>   cli = "run --bin redbox-cli --"
>   server = "run --bin redbox-server --"
>   ```
>
>   直接运行 `cargo cli` 或 `cargo server` 即可。

## 建立连接

添加必要的项目依赖：

```shell
cargo add tokio --features full
```

- 编辑 `redbox-server.rs`

```rust
use redbox::server;

#[tokio::main]
async fn main() {
    let port = 63790; // Redis at 6379
    let addr = format!("127.0.0.1:{}", port);
    if let Err(err) = server::run(&addr).await {
        panic!("Redbox failed: {}", err);
    }
}
```

考虑到 Redis 默认端口为 `6379`，为不影响使用，我们就监听 `63790` 端口吧！

- 编辑 `lib.rs`

```rust
pub mod server;
```

暴露 `server` 模块。

- 编辑 `server.rs`

上述都是铺垫，现在才算进入正题。

我们在 `run` 函数中创建一个 `TcpListener`，绑定到给定端口。

```rust
use std::io;

use tokio::net::TcpListener;
use tokio::net::TcpStream;

pub async fn run(addr: &str) -> io::Result<()> {
    let listener = TcpListener::bind(addr).await?;

    println!("Redbox started at {}", addr);
    while let Ok((stream, addr)) = listener.accept().await {
        println!("Accept from {}", addr);
        process(stream).await?;
    }

    println!("Redbox stopped");

    Ok(())
}

async fn process(mut stream: TcpStream) -> io::Result<()> {
    todo!()
}
```

现在 `process` 函数该怎么处理传入的 `TcpStream`？

Redis 客户端和服务器使用 RESP（**RE**dis **S**erialization **P**rotocol）协议来通信。
我们虽然尚不清楚协议的具体内容，但知道由 `redis-cli` 传入的数据一定满足该协议。
只需要把接收到的数据原路返回，即可完成服务应答！

开辟缓冲区，然后导入 `tokio::io::{AsyncReadExt,AsyncWriteExt}` 性状，赋予 `TcpStream` 读写缓冲区的能力：

```rust
use std::io;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

async fn process(mut stream: TcpStream) -> io::Result<()> {
    let mut buf = vec![0; 1024];

    while let Ok(n) = stream.read(&mut buf).await {
        if n > 0 {
            println!("Received {} bytes", n);
            stream.write_all(&buf[..n]).await?;
        }
    }

    Ok(())
}
```

读到什么，就返回什么！

现在检查一下成果：执行 `cargo run`，没有报错！打开新终端，执行 `redis-cli -p 63790` 连接服务器，成功进入！执行几条命令试试：

```redis
127.0.0.1:63790> KEYS *
1) "KEYS"
2) "*"
127.0.0.1:63790> SET redbox ok
1) "SET"
2) "redbox"
3) "ok"
127.0.0.1:63790>
```

符合预期，没有问题！

别关窗口，我们再连一个试试！

## 再连一个试试！

打开新终端，再次执行 `redis-cli -p 63790` ……咦？怎么没反应？

我们捋一捋：`process` 不停的读写，是个无限循环，那么 `run` 就没有机会处理第二个连接！

怎么办？我们把 `process` 放入另一个异步任务中：

```rust
while let Ok((stream, addr)) = listener.accept().await {
    tokio::spawn(async move {
        println!("Accept from {}", addr);
        process(stream).await
    });
}
```

再连一个试试，一切正常！

## 打印日志

添加必要的项目依赖：

```shell
cargo add log env-logger
```
