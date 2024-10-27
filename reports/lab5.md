# 第八章实验报告

## 荣誉准则

1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 **以下各位** 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

   > 无

2. 此外，我也参考了 **以下资料** ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

   > 无

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。

## 编程题

简单总结你实现的功能（200字以内，不要贴代码）。

1. mutex 的testset较简单，就偷懒了下，只实现了不能持有自己持有过的锁。
1. 按照文档实现了 sem 的死锁检测算法。给 PCB 新增了 Available 和 Need 矩阵（都是`Vec<Vec<usize>>`类型），在相关操作时扩容 Available 和 Need 矩阵，如在 Create 的时候为 Available 矩阵中每个 task 加一列。在 up 操作时 维护 Allocation 矩阵，down 时维护 Allocation 和 Need 矩阵(如果获取不到资源，需要加 Need 矩阵，否则加 Allocation 矩阵)。在 down 操作执行前判断死锁。


## 问答题

1. 在我们的多线程实现中，当主线程 (即 0 号线程) 退出时，视为整个进程退出， 此时需要结束该进程管理的所有线程并回收其资源。
   - 需要回收的资源有哪些？
   
     所有线程用户栈和 Trap 上下文，所有线程内核栈和内核中的线程控制块、描述符等资源，和进程占有的进程控制块、memory_set中的页面、fd_table等资源。
   
   - 其他线程的 TaskControlBlock 可能在哪些位置被引用，分别是否需要回收，为什么？
   
     可能在调度队列和锁引用，需要回收，如果不回收可能会导致进程永远处于阻塞，从而导致内存泄漏。
   
2. 对比以下两种 `Mutex.unlock` 的实现，二者有什么区别？这些区别可能会导致什么问题？

```
impl Mutex for Mutex1 {
    fn unlock(&self) {
        let mut mutex_inner = self.inner.exclusive_access();
        assert!(mutex_inner.locked);
        mutex_inner.locked = false;
        if let Some(waking_task) = mutex_inner.wait_queue.pop_front() {
            add_task(waking_task);
        }
    }
}

impl Mutex for Mutex2 {
    fn unlock(&self) {
        let mut mutex_inner = self.inner.exclusive_access();
        assert!(mutex_inner.locked);
        if let Some(waking_task) = mutex_inner.wait_queue.pop_front() {
            add_task(waking_task);
        } else {
            mutex_inner.locked = false;
        }
    }
}
```

**Mutex1**：在解锁过程中先将 `locked` 标志位设置为 false，然后再尝试从等待队列中取出任务并唤醒。

**Mutex2**：在解锁过程中先尝试从等待队列中取出任务并唤醒，只有当等待队列为空时才将 `locked` 标志位设置为 false。

lock函数会先检查locked是否为false，如果是则继续suspend，所以mutex2会导致无法正常获取锁。

## 你对本次实验设计及难度/工作量的看法，以及有哪些需要改进的地方

希望增强测试集，同时详细解释下死锁检测和死锁避免等的知识点。