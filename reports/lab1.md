# ch3 实验报告

## 编程作业

将本次实验需要的额外信息写入任务管理块 `TaskControlBlock` 的一个结构体成员，并在 `TaskManager` 中添加相关调用接口

### 任务状态

要获取被调度任务，只需从 `TaskManager` 中直接获取即可

### 系统调用

在 `syscall` 调用系统接口的位置调用即可

### 运行时间

在启动任务时记录当前时间，分别在 `run_first_task` 和 `run_next_task` 记录，在 `run_next_task` 中需要判断防止重复进入重置时间。

在计算时间时，需要对 `get_time_us` 的结果进一步处理

## 简答作业

1. SBI 版本：`RustSBI-QEMU Version 0.2.0-alpha.2`
   - `ch2b_bad_address`：产生 `Trap::Exception(Exception::StoreFault)` 异常：`PageFault in application, bad addr = 0x0, bad instruction = 0x804003a4, kernel killed it.`
   - `ch2b_bad_instructions`： 产生 `Trap::Exception(Exception::IllegalInstruction)` 异常：`IllegalInstruction in application, kernel killed it.`
   - `ch2b_bad_register`：产生 `Trap::Exception(Exception::IllegalInstruction)` 异常：`IllegalInstruction in application, kernel killed it.`

2. `trap.S`
   1. 进入 `__restore` 时，`a0` 寄存器保存的是上一个任务相关信息（`*current_task_cx_ptr`）。其使用场景包括：
      1. 运行第一个程序，从 S 特权级跳转到 U 特权级
      2. 任务调度，切换不同任务时
   2. 特殊处理了 `sstatus`，`sepc`，`sscratch` 三个寄存器，作用分别为：
      - `sstatus`：`SPP` 等字段给出 Trap 发生之前 CPU 处在哪个特权级（S/U）等信息
      - `sepc`： 当 Trap 是一个异常的时候，记录 Trap 发生之前执行的最后一条指令的地址
      - `sscratch`：指向用户栈顶
   3. `x2` 是 `sp`，指向内核栈，而用户栈通过 `sscratch` 保存；`x4` 是 `tp`，除非手动使用，一般不会被用到
   4. 该指令用于交换 `sp` 和 `sscratch` 内容，交换后 `sp` 指向用户栈，`sscratch` 指向内核栈
   5. 第 61 行，`sret` 指令用于返回 U 特权级
   6. 执行后 `sp` 指向内核栈，`sscratch` 指向用户栈
   7. 第 38 行 `call trap_handler` 指令

# 荣誉准则

1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 **以下各位** 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

   > * 一个莫生的人：询问实验报告以  rCore-Tutorial-Book-v3 文档还是 rCore-Camp-Guide-2024A 文档要求写
   > * gogoing：有关本地测试通过，但远程不通过的问题

2. 此外，我也参考了 **以下资料** ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

   > *《rCore-Camp-Guide 2024A 文档》*

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。