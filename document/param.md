# 解析命令行参数

## 获取参数

```rust
//获取参数 std::env::args()

let pattern = std::env::args().nth(1).expect("no pattern given");
let path = std::env::args().nth(2).expect("no path given");
```

举例:
rucli run 1+1

第一个参数: rucli
第二个参数: run
第三个参数: 1+1


## 解析参数

使用 StructOpt 解析 CLI 参数