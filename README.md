# german-num
A german number exercise written in Rust

## Update

### Version v0.2.16 
 * Add online installer

## Online Installation
```
curl -fsSL https://dl.leolio.page/german-num/ | python3
```
For instance, if you are using mac, the download address should be:
```
https://dl.leolio.page/german-num/aarch64-clang/0.2/german-num.tar.gz
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
# german-num v.0.2.10
# Aktualisiert am: 17.07.2023
#
############################

Geben Sie 'exit' ein, um das Programm zu beenden!

== Erste Stufe L1 ==
1) Was ist die Zahl 4 auf Deutsch? vier
» Richtig!
2) Was ist die Zahl 9 auf Deutsch? neun
» Richtig!
3) Was ist die Zahl 6 auf Deutsch? sechs
» Richtig!
4) Was ist die Zahl 4 auf Deutsch? vier
» Richtig!
5) Was ist die Zahl 6 auf Deutsch? sechs
» Richtig!
6) Was ist die Zahl 3 auf Deutsch? drei
» Richtig!
7) Was ist die Zahl 6 auf Deutsch?
```

Happy Programming!