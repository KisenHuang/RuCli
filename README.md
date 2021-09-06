# rucli

rust 命令行应用程序

## 简介
目前对于Android跨平台使用打包，使用的都是jni。
我们不仅需要手动编写rust的jni文件，我们还需要手动编写java的接口文件。

这里我们定义一个命令行工具，实现通过rust的jni文件生成java接口文件的工具。

## 实现步骤

* 解析命令行参数
* 定义指令
* 实现操作

## 使用方式

### 编译打包
cargo build

在项目根目录中 target->debug ，找到 rucli，这就是可执行程序。

### 拷贝 rucli

将rucli拷贝到需要处理的目录

### 执行命令

rucli jni <需要操作的文件路径>