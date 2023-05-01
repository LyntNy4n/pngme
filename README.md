# PNGme

该项目基于 [PNGme: An Intermediate Rust Project](https://picklenerd.github.io/pngme_book/introduction.html) 

This project is based on [PNGme: An Intermediate Rust Project](https://picklenerd.github.io/pngme_book/introduction.html)

这是一个命令行程序，可以在 PNG 文件中隐藏秘密消息,程序将有四个命令:

1. 
   将消息编码为 PNG 文件
2. 
   解码存储在 PNG 文件中的消息
3. 
   从 PNG 文件中删除消息
4. 
   打印可搜索消息的 PNG 块列表

This is a command line program that lets you hide secret messages in PNG files. This program will have four commands:

1. Encode a message into a PNG file
2. Decode a message stored in a PNG file
3. Remove a message from a PNG file
4. Print a list of PNG chunks that can be searched for messages

## 用法 Usuge

```bash
# add a custom chunk in dice.png
pngme encode ./dice.png ruSt "This is a secret message!

pngme decode ./dice.png ruSt
> "This is a secret message!

# remove the 'ruSt' chunk in dice.png
pngme remove ./dice.png ruSt

# print all of the chunks of dice.png
pngme print ./dice.png
```

