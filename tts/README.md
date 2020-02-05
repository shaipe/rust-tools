定时任务 -- tts
===

一个可以配置的定时任务,编译成可执行文件后,配置到linux内核的服务中进行运行

### 需求

1、定时触发 尝试3次
   如 2020-01-17 17:30:00 定时触发
2、循环触发
   如 每10分钟 访问URL接口
3、时段触发
   如 每天 02:00 - 03:00 ，每3分请求一次

### 参考资料

- [定时器](https://rustlang-cn.org/crates/tokio/docs/going-deeper/timers.html)
- [42 定时器](https://rustcc.github.io/rust-by-example/timers.html)
- [Rust有相当于Python的线程.Timer](http://www.voidcn.com/article/p-qidzjxoa-bvs.html)

## 运行配置说明

