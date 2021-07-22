# Lab5 Report

## 实验内容

- 实现 `sys_getpid`、`sys_fork`、`sys_exec`、`sys_waitpid` 系统调用，实现进程控制。
- 实现 `sys_read` 系统调用，可以运行 usershell。
- 实现 `sys_spawn` 系统调用。

## 运行截图

![result-5](./result-5.gif)
_5_

![result-5_only](./result-5_only.gif)
_5_only_

![result-usershell](./result-usershell.gif)
_usershell_

## 问答作业

1. fork + exec 的一个比较大的问题是 fork 之后的内存页/文件等资源完全没有使用就废弃了，针对这一点，有什么改进策略？
2. 其实使用了题(1)的策略之后，fork + exec 所带来的无效资源的问题已经基本被解决了，但是今年来 fork 还是在被不断的批判，那么到底是什么正在“杀死”fork？可以参考[论文](https://www.microsoft.com/en-us/research/uploads/prod/2019/04/fork-hotos19.pdf) 。
3. fork 当年被设计并称道肯定是有其好处的。请使用**带初始参数**的 spawn 重写如下 fork 程序，然后描述 fork 有那些好处。注意:使用“伪代码”传达意思即可，spawn 接口可以自定义。可以写多个文件。
4. 描述进程执行的几种状态，以及 fork/exec/wait/exit 对于状态的影响。
