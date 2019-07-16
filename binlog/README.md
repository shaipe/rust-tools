mysql -- binlog分析工具
===

采用rust编写的binlog日志分析工具

## binlog 转 sql

在mysql的环境下执行以下语句

```
# 将binlog文件转sql并进行base64解码
mysqlbinlog --base64-output=decode-rows -v mysql-bin.003757>003757-1.sql
```

