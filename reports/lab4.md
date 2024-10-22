# 第六-七章实验报告

## 荣誉准则

1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 **以下各位** 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

   > 无

2. 此外，我也参考了 **以下资料** ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

   > 无

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。

## 编程题

简单总结你实现的功能（200字以内，不要贴代码）。

1. 给 trait file 加上stat函数，给vfs inode加了一个get_id函数获取inode的id，使用get_nlink函数获得inode_id与本文件一样的文件。
1. link：把原文件的inode_id拿到，new一个新name到旧id的dirent。unlink：首先找inode，删掉dirent，如果删掉的是最后的dirent就要先用clear函数删掉当前inode的数据，然后用dealloc_inode（EasyFileSystem.inode_bitmap.dealloc）删掉inode的位图。


## 问答题

1. （ch6）在我们的easy-fs中，root inode起着什么作用？如果root inode中的内容损坏了，会发生什么？

   root inode 存放了根目录的信息，在easy-fs里面根目录的目录项存放了所有文件的 inode 编号。如果损坏了会导致虽然所有文件都在硬盘上，但是没有文件名对应的 inode 编号，这些文件就索引不到了。

2. （ch7）举出使用 pipe 的一个实际应用的例子。

   在命令的输出中找指定字符串：`make run | grep FAIL`

3. 如果需要在多个进程间互相通信，则需要为每一对进程建立一个管道，非常繁琐，请设计一个更易用的多进程通信机制。

   共享内存：多个进程可以同时mmap同一段物理内存来共享数据，并使用额外的同步机制来控制进程对共享内存的访问。