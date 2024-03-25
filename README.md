# calc
Simple calculator made in RUST (with float values)

After seing the calc made in Python by GaryExplains and the Shunting-yard example : 
https://github.com/garyexplains/examples/tree/master/shunting-yard

I made a Rust one :)

```
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.12s
     Running `target\debug\calc.exe`
Enter infix expression (ex: -->3 + 4 * 2 / ( 1 - 5 ) ^ 2 ^ 3<--)
3 + 4 * 2 / ( 1 - 5 ) ^ 2 ^ 3
TOKENS: ["3", "+", "4", "*", "2", "/", "(", "1", "-", "5", ")", "^", "2", "^", "3"]
Postfix expression: 3 4 2 * 1 5 - 2 ^ 3 ^ / +
Result: 3.001953125
```
