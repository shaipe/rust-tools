# rust-tools

采用rust-lang编写一些常用的工具,带着目标去学习

## deployer (部署工具)

主要用于对文件进行按日志进行提取打包并上传到服务

## tts (定时任务工具)

以一个服务的形式放置于系统中,用于按设定的时间执行网络请求任务

## binlog

mysql数据库日志分析,把binlog日志转换为sql后,此工具体把sql文本文件转换成执行语句写入mongodb数据库中

## coder

一个代码生成器,可以根据mysql的数据库表结构进行代码生成



## rdl

用Rust写一个dll在C#中进行调用的测试

## tool 单个工具

基于examples实现单一功能
1. tust2sql: 提取Rust语言中的struct结构生成sql语句 --example 
2. file_category: 对目录下的文件根据类型进行分类 
3. file_rename: 对目录下的文件进行重重命名,支持正则替换

## wmps

用rust写一个windows 服务,在examples中有关于服务管理的工具.

## 学习资料

- [Rust 学习指南 - Rust 高级结构](https://www.codemore.top/p/234853bd-4f89-3df9-ae18-994110faf023)

### tip ssl error
```bash
git config --global http.sslVerify false
```