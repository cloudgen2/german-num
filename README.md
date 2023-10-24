# german-num
A german number exercise written in Rust. This project uses the online-downloader script in https://github.com/cloudgen2/online-installer


## Online Installation
```
curl -fsSL https://dl.leolio.page/german-num/ | python3
```
For instance, if you are using mac, the download address should be:
```
https://dl.leolio.page/german-num/aarch64-clang/0.4/german-num.tar.gz
```

## Run the source code
```
cargo run
```

## Build release
```
cargo build --release
rm -rf ~/.local/bin/spanish-num
cp target/release/german-num  ~/.local/bin/
```

## Execution example

```
############################
#
# german-num v.0.4.0
# Aktualisiert am: 14.10.2023
#
############################

Geben Sie 'exit' ein, um das Programm zu beenden!

== Erste Stufe L1 ==
1) Was ist die Zahl 1 auf Deutsch? eins
» Richtig!
2) Was ist die Zahl 1 auf Deutsch? eins
» Richtig!
3) Was ist die Zahl 1 auf Deutsch? eins
» Richtig!
4) Was ist die Zahl 8 auf Deutsch? acht
» Richtig!
5) Was ist die Zahl 5 auf Deutsch? fünf
» Richtig!
6) Was ist die Zahl 2 auf Deutsch? zwei
» Richtig!
7) Was ist die Zahl 6 auf Deutsch? sechs
» Richtig!
8) Was ist die Zahl 6 auf Deutsch? sechs
» Richtig!
9) Was ist die Zahl 2 auf Deutsch? zwei
» Richtig!
10) Was ist die Zahl 4 auf Deutsch? vier
» Richtig!
12) Was ist das? ( 1🍎 )? Das ist ein Apfel.
» Richtig!
13) Was ist das? ( 10🍎 )? Das sind zehn Äpfel.
» Richtig!
```

Happy Programming!
