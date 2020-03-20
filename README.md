# 翻译工具

使用有道翻译，替换`APP_KEY`和`APP_SECRET`为自己的。
```rust
const APP_KEY: &'static str = "xxx";
const APP_SECRET: &'static str = "xxx";
```

## 使用
```cmd
> cargo run hello world
    Finished dev [unoptimized + debuginfo] target(s) in 0.20s
     Running `target\debug\fy.exe hello world`
原文: hello world
翻译: 你好，世界
释义:
    hello world: 你好世界, 开始, 别来无恙, 举个例子
    Hello Kitty World: 凯蒂猫气球世界
    Hello Cold World: 每天一歌
```

## 构建发布版本
```cmd
> cargo build --release
```
复制`fy.exe`到`PATH`（添加到`PATH`）目录下。

命令行使用:
```cmd
> fy hello world
原文: hello world
翻译: 你好，世界
释义:
    hello world: 你好世界, 开始, 别来无恙, 举个例子
    Hello Kitty World: 凯蒂猫气球世界
    Hello Cold World: 每天一歌
```