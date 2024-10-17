# 第三章实验报告

## 荣誉准则

1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 **以下各位** 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

   > 无

2. 此外，我也参考了 **以下资料** ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

   > 无

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。

## 编程题

简单总结你实现的功能（200字以内，不要贴代码）。

在tcb可变部分中记录了初次调度的时间和任务系统调用次数。其中，初次调度时间在run_first_task和run_next_task函数记录，系统调用次数在内核中使用与系统调用接口相同的桶计数方法维护，并在syscall接口调用累加的函数。

## 问答题

1. 正确进入 U 态后，程序的特征还应有：使用 S 态特权指令，访问 S 态寄存器后会报错。 请同学们可以自行测试这些内容（运行 [三个 bad 测例 (ch2b_bad_*.rs)](https://github.com/LearningOS/rCore-Tutorial-Test-2024A/tree/master/src/bin) ）， 描述程序出错行为，同时注意注明你使用的 sbi 及其版本。

   程序出错行为是触发trap， CPU 切换到 S 特权级并跳转到 `stvec` 所指示的位置，即系统的trap_handler函数，bad_address程序报错 `PageFault in application, bad addr = 0x0, bad instruction = 0x804003a4, kernel killed it.`，另外两个程序报错`IllegalInstruction in application, kernel killed it`。

   使用的sbi：RustSBI-QEMU Version 0.2.0-alpha.3

2. 深入理解 [trap.S](https://github.com/LearningOS/rCore-Camp-Code-2024A/blob/ch3/os/src/trap/trap.S) 中两个函数 `__alltraps` 和 `__restore` 的作用，并回答如下问题:

   1. L40：刚进入 `__restore` 时，`a0` 代表了什么值。请指出 `__restore` 的两种使用情景。

      调用__restore的goto_restore函数第一个参数是kstack_ptr，所以a0代表内核栈压入 Trap 上下文之后的栈顶。

      使用场景：1. trap后从内核态切换回用户态和2. 需要跳转到应用程序入口执行应用程序时。

   2. L43-L48：这几行汇编代码特殊处理了哪些寄存器？这些寄存器的的值对于进入用户态有何意义？请分别解释。

      ```
      ld t0, 32*8(sp)
      ld t1, 33*8(sp)
      ld t2, 2*8(sp)
      csrw sstatus, t0
      csrw sepc, t1
      csrw sscratch, t2
      ```
      
      sstatus: SPP 等字段给出 Trap 发生之前 CPU 处在哪个特权级（S/U）等信息，恢复用户程序的特权级和CPU行为。
      
      sepc: 恢复用户程序的pc，sret指令会从该pc继续执行。
      
      sscratch: 保存用户栈指针的栈顶，用户态需要将sp置为该值。
      
   3. L50-L56：为何跳过了 `x2` 和 `x4`？
   
   ```
   ld x1, 1*8(sp)
   ld x3, 3*8(sp)
   .set n, 5
   .rept 27
      LOAD_GP %n
      .set n, n+1
   .endr
   ```
   
      x2 是sp寄存器，用户栈的 sp 已经保存在 sscratch 中
   
      x4 一般也不会被用到
   
   4. L60：该指令之后，`sp` 和 `sscratch` 中的值分别有什么意义？
   
   ```
   csrrw sp, sscratch, sp
   ```
   
      该指令交换了sp和sscratch的值，该指令后sp存的是用户态的栈顶指针，sscratch是内核态的栈顶指针。
   
   5. `__restore`：中发生状态切换在哪一条指令？为何该指令执行之后会进入用户态？
   
      sret指令。该指令会将当前的特权级按照 `sstatus` 的 `SPP` 字段设置为 U 或者 S，这是硬件自动完成的。
   
   6. L13：该指令之后，`sp` 和 `sscratch` 中的值分别有什么意义？
   
   ```
   csrrw sp, sscratch, sp
   ```
   
      该指令交换了sp和sscratch的值，该指令后sp存的是内核态的栈顶指针，sscratch是用户态的栈顶指针。
   
   7. 从 U 态进入 S 态是哪一条指令发生的？
   
      是用户态的trap类指令发生的。
