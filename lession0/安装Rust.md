# 安装 Rust

参考官网自助安装，这绝对难不倒你：[官网安装](https://www.rust-lang.org/learn/get-started)

> 知识点0：推荐使用 rustup 安装，而非 brew，后续对 Rust 版本的管理 rustup 更为官方。

# 运行

1、创建第一个 demo：

```
cargo new hello-rust
```

2、翻看目录结构

3、运行 demo，观察打印信息

```
Cargo run
```

4、再查看工程目录，注意体会变化：

/target/debug 目录下已经产出了一个同名二进制文件：hello-rust，直接访问即可运行，并看到同样的打印效果：

```
./target/debug/hello-rust
```