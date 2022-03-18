# 第 2 章：解析 RESP 数据

## RESP 协议

> 具体可参考 <https://redis.io/topics/protocol>

### 五类数据

顾名思义，RESP（**RE**dis **S**erialization **P**rotocol）协议是一种序列化协议，支持五类数据。

每种类型都以特定模式开头，用**回车换行**（`\r\n`，CRLF）标记数据结尾：

1. `Integer`，形如 `:{i64}\r\n`；
1. `Error`，形如 `-{str}\r\n`；
1. `SimpleString`，形如 `+{str}\r\n`；
1. `BulkString`，形如 `${len}\r\n{bytes}\r\n`；
1. `Array`，形如 `*{len}\r\n{elements}`。

此外，还有两个表示 `Null` 的特例：

1. `$-1\r\n`，即 `Null Bulk String`；
1. `*-1\r\n`，即 `Null Array`。

### 请求与响应时的差异

Redis 在处理请求、响应时所用数据是有侧重的：

- 客户端发起请求时，发送的永远是由 `BulkString` 组成的 `Array`；
- 服务器作出响应时，会根据命令实现，发送相应类型。

## 解析数据

## 解析命令
