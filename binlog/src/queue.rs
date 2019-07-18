#[derive(Debug)]
pub struct Queue<T> {
    qdata: Vec<T>,
}

impl <T> Queue<T> {

    /// 新创建一个队列对象
    pub fn new() -> Self {
        Queue{qdata: Vec::new()}
    }

    /// 添加一对象到队列中
    pub fn push(&mut self, item:T) {
        self.qdata.push(item);
    }

    /// 获取队列中的第一个对象
    pub fn pop(&mut self) -> T{
        self.qdata.remove(0)
    }

    /// 获取队列的长度
    pub fn len(&self) -> usize {
        self.qdata.len()
    }
}
