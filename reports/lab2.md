# Lab2 Report

## 实验内容

- 实现 `sys_write` 和 `sys_exit` 系统调用。
- 为 `sys_write` 增加安全性检查。

## 运行截图

![result-2](./result-2.gif)
_2_

![result-2_bad](./result-2_bad.gif)
_2_bad_

## 问答作业

1. 正确进入 U 态后，程序的特征还应有：使用 S 态特权指令，访问 S 态寄存器后会报错。目前由于一些其他原因，这些问题不太好测试，请同学们可以自行测试这些内容（参考[前三个测例](https://github.com/DeathWish5/rCore_tutorial_tests/tree/master/user/src/bin)），描述程序出错行为，同时注意注明你使用的 sbi 及其版本。

   **答**：`_ch2_bad_instruction`：IllegalInstruction，`_ch2_bad_register`：IllegalInstruction，`_ch2t_bad_address`：PageFault（环境：rustsbi-qemu[d4968dd2]）。

2. 请结合用例理解 [trap.S](https://github.com/rcore-os/rCore-Tutorial-v3/blob/ch2/os/src/trap/trap.S) 中两个函数 `__alltraps` 和 `__restore` 的作用，并回答如下几个问题:

   1. L40：刚进入 `__restore` 时，`a0` 代表了什么值。请指出 `__restore` 的两种使用情景。

      **答**：内核栈栈顶。使用情景：① 恢复 Trap 上下文，② CPU 特权级降级。

   2. L46-L51：这几行汇编代码特殊处理了哪些寄存器？这些寄存器的的值对于进入用户态有何意义？请分别解释。

      **答**：`t0`：`sstatus`，`SPP` 等字段给出 Trap 发生之前 CPU 处在哪个特权级（S/U）等信息；`t1`：`sepc`，记录 Trap 发生之前执行的最后一条指令的地址；`t2`：用户栈栈顶。

   3. L53-L59：为何跳过了 `x2` 和 `x4`？

      **答**：`x2` 为 `sp`（`Stack pointer`），`x4` 为 `tp`（`Thread pointer`）。`x4` 没有使用，`x2` 之后会更改。

   4. L63：该指令之后，`sp` 和 `sscratch` 中的值分别有什么意义？

      **答**：`sp`：用户栈栈顶，`sscratch`：内核栈栈顶。

   5. `__restore`：中发生状态切换在哪一条指令？为何该指令执行之后会进入用户态？

      **答**：`sret`。因为此时 CPU 的特权级被设置为 U（`sstatus` 的 `SPP`），并跳转进入 S 态前的下一条指令（`sepc`）。

   6. L13：该指令之后，`sp` 和 `sscratch` 中的值分别有什么意义？

      **答**：`sp`：内核栈栈顶，`sscratch`：用户栈栈顶。

   7. 从 U 态进入 S 态是哪一条指令发生的？

      **答**：`call trap_handler`。

3. 程序陷入内核的原因有中断和异常（系统调用），请问 riscv64 支持哪些中断/异常？如何判断进入内核是由于中断还是异常？描述陷入内核时的几个重要寄存器及其值。

   **答**：支持的中断/异常：

   | Interrupt | Exception Code | Description                    |
   | --------: | -------------: | :----------------------------- |
   |         1 |              0 | _Reserved_                     |
   |         1 |              1 | Supervisor software interrupt  |
   |         1 |            2–4 | _Reserved_                     |
   |         1 |              5 | Supervisor timer interrupt     |
   |         1 |            6–8 | _Reserved_                     |
   |         1 |              9 | Supervisor external interrupt  |
   |         1 |          10–15 | _Reserved_                     |
   |         1 |            ≥16 | _Designated for platform use_  |
   |         0 |              0 | Instruction address misaligned |
   |         0 |              1 | Instruction access fault       |
   |         0 |              2 | Illegal instruction            |
   |         0 |              3 | Breakpoint                     |
   |         0 |              4 | Load address misaligned        |
   |         0 |              5 | Load access fault              |
   |         0 |              6 | Store/AMO address misaligned   |
   |         0 |              7 | Store/AMO access fault         |
   |         0 |              8 | Environment call from U-mode   |
   |         0 |              9 | Environment call from S-mode   |
   |         0 |          10–11 | _Reserved_                     |
   |         0 |             12 | Instruction page fault         |
   |         0 |             13 | Load page fault                |
   |         0 |             14 | _Reserved_                     |
   |         0 |             15 | Store/AMO page fault           |
   |         0 |          16–23 | _Reserved_                     |
   |         0 |          24–31 | _Designated for custom use_    |
   |         0 |          32–47 | _Reserved_                     |
   |         0 |          48–63 | _Designated for custom use_    |
   |         0 |            ≥64 | _Reserved_                     |

   | CSR 名  |                      该 CSR 与 Trap 相关的功能                       |
   | :-----: | :------------------------------------------------------------------: |
   | sstatus |    `SPP` 等字段给出 Trap 发生之前 CPU 处在哪个特权级（S/U）等信息    |
   |  sepc   | 当 Trap 是一个异常的时候，记录 Trap 发生之前执行的最后一条指令的地址 |
   | scause  |                           描述 Trap 的原因                           |
   |  stval  |                          给出 Trap 附加信息                          |
   |  stvec  |                     控制 Trap 处理代码的入口地址                     |

4. 对于任何中断，`__alltraps` 中都需要保存所有寄存器吗？你有没有想到一些加速 `__alltraps` 的方法？简单描述你的想法。

   **答**：只需要保持需要使用且会发生变化的寄存器。
