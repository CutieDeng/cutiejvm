## Chapter 2 The structure of the Java Virtual Machine 

该文档描述了一个抽象的机器规范，但并不指定任何一种 JVM 实现的具体行为。

为了正确地实现 JVM, 你必须能够分析 `class` 的文件格式并且执行在其中定义的行为。
实现细节不是 JVM 规范的一部分——规范不会约束实现者的创造力。
例如，运行时数据区域的内存布局，垃圾收集算法 (GC *garbage-colleciton*) 的选择，以及设计在指令集内部的优化，比方说，将其再次翻译为机器码。这些设计的使用都是实现者的自由。

参考 *The Unicode Standard, Version 13.0*, 本规范中，所有关于 Unicode 的描述都能在 https://www.unicode.org/ 中找到。

### Chapter 2.1 `class` 文件格式

JVM 能执行的字节码表示为一种平台无关的二进制格式，往往会存储在文件中，称为 `class` 文件格式。
该文件格式精确地定义了类和接口的表式形式，包括诸如字节序这样在某一平台会被视为理所当然的细节。

Chapter 4 包括了 `class` 格式的细节。

### Chapter 2.2 数据类型

像 Java 语言一样，Java Virtual Machine 把数据为了两种类型：基本类型和引用类型。
相应地，两种类型的数据都可以被存储至变量中，进行参数传递，通过方法（ *method* 也可以理解成「函数 ) 进行返回，或被操作。

JVM 希望几乎所有类型检查都能借助编译器在开始运行前完成，而不必再让 JVM 自己来检查。

基础类型的值不需要在运行时被标记上类型，也不需要与引用类型区分开。
取而代之的，是 JVM 的指令集本身在定义之时，便隐式地描述了操作数的类型。
举个例子，指令 `iadd`, `ladd`, `fadd`, `dadd` 是 JVM 中用于进行两个数值类型的加法运算的指令，其会得到一个同样是数值类型的结果。而不同的指令针对性地描述了不同的操作数类型：`int`, `long`, `float`, `double`. 
在第 11 小节中，我们会有更详细的指令对数据类型的支持描述汇总。

### Chapter 2.3 基本类型、值

基本类型包括：数值类型 ( `byte` , `short` , `int` , `long` , `char` , `float` , `double` ), `boolean` 类型，`returnAddress` 类型。

整型数值类型使用补码的方式进行编码（除 `char` 类型是 16-bit unsigned 编码模式）

其中，各类型的编码长度如下：

|type|bit width|
|--|--|
|`byte`|8-bit|
|`short`|16-bit|
|`int`|32-bit|
|`long`|64-bit|

所有整型数值类型的默认值都是零。

`float` 类型遵循 `32-bit IEEE 754 binary32` 的值表示规范，`double` 为 `64-bit IEEE 754 binary64`. 
而它们的默认值为正零。(positive zero) 

`boolean` 类型（常称 *布尔类型* ） 只有两个值：`true` 和 `false`. 
默认值是 `false`. 

> 在第一版的 *Java® Virtual Machine Specification* 中没有考虑过 `boolean` 会是 JVM 中的一个类型。然而，当时 `boolean` 在 JVM 中依旧有一定的支持。
> 在第二版中，规范澄清了这个问题——将 `boolean` 明确定义为一种类型。

最后，`returnAddress` 类型是对 JVM 指令位置的描述（类比 C 的函数指针，或汇编的 .text 符号）。
所有类型中，只有 `returnAddress` 类型不是直接在 Java 语言里出现的类型。

#### Chapter 2.3.1 整型类型表示范围

`byte` : -128 ~ 127 
`short` : -32768 ~ 32767 
`int` : -2147483648 ~ 2147483647
`long` : -9223372036854775808 ~ 9223372036854775807 
`char` : 0 ~ 65535 

### Chapter 2.3.2 浮点数类型表示范围

[[TODO: 不想翻译了，又不用自己写浮点数运算的硬件。]]

### Chapter 2.3.3  `returnAddress` 类型

在 JVM 中，三条指令 `jsr`, `ret`, `jsr_w` 都需要使用到 `returnAddress` 类型。
该类型的值是指向 Java 指令的指针（，如果用 C 的方言来说的话）。
和平凡的数值类型不同，该类型既不与 Java 编程语言的任何一种类型相关，也不能在程序运行的过程中修改。（笔者注：这里的「修改」应该是指不允许进行指针算术。）

### Chapter 2.3.4 `boolean` 类型

虽然说，JVM 规定 `boolean` 是一种类型，但 Java Virtual Machine 其实只提供了非常受限的支持。
没有任何一条 JVM 指令单独地专门用于进行 `boolean` 类型相关的运算，而在实际的 JVM 运行中，布尔表达式的运算则被编译成 JVM `int` 类型间的运算。

不过呢，JVM 的确真的支持 `boolean` 类型，比如说 `boolean` 数组。
数组创建指令 `newarray` 可以创建一个 `boolean` 数组。
而数组中 `boolean` 值的访问和修改可以通过 `byte` 数组的操作指令 `baload` 和 `bastore`. 

> 在 Oracle 的 JVM 实现中，Java 语言定义的 `boolean` 数组在 JVM 中被编码为 `byte` 数组，即每个 `boolean` 类型都占 8 bit. 

### Chapter 2.4 引用类型

引用类型可以被划分为三大类：`class` 类型（类），`array` 类型（数组），`interface` 类型（接口）。
而它们的值，则是动态创建的对象、数组，或者是实现了某个接口的对象（或数组）。

数组类型描述里不包含该数组的长度，但会包含其中的成员类型。
我们还会强调数组的「元素类型」，它被递归的定义为数组的成员类型的元素类型，除非该数组的成员不再是数组。

所以，数组的元素类型，要么是基本类型，要么是 `class` 类型，要么是 `interface` 类型。

而一个引用类型的值，可以是 `null` (空值，在编程术语中常表达为不存在意。)。
*值得注意的是，`null` 引用没有一个运行时维护的类型，但它可以强制转化为任一类型。*
引用类型的默认值是 `null`. 

本规范并不规定 `null` 的编码方式。

### Chapter 2.5 运行时数据区

Java Virtual Machine 定义了不同的运行时数据区，以便在程序执行的过程中使用。
其中，有些数据区会在 JVM 初始化时创建，而在 JVM 退出时销毁。（值得注意的是，JVM 会在退出时主动回收，这是因为除了 JVM 单独运行在一个进程的情形，还有一些情形是 JVM 被外挂在某个进程内部，如果 JVM 不回收该内存，将会导致其销毁后原进程内存泄漏。） 
另一些数据区则是被各线程自身持有。这些数据区将会在该线程创建时同时创建，而在该线程退出时一同销毁。

#### Chpater 2.5.1 pc 寄存器

*pc 寄存器: 全名 program counter register, 用于描述当前 CPU 应当执行的指令位置。*

JVM 支持同时运行多个线程，因此各 JVM 线程都有它自己的 pc 寄存器。
在任意时刻，每个 JVM 线程都在执行某个方法里面的代码，而这个方法，称之为该线程的 *current method*. 
如果该方法不是 **native**, 则 pc 寄存器将会包含当前 JVM 接下来要执行的指令位置。
如果是，则 pc 的值是未定义的。(即，规范不要求该值一定是某个值，也不要求该值必须在不同的执行时间相同，不要求该值满足某种序列或规律。) 
pc 寄存器的位宽足以保存一个 `returnAddress` 类型的值或者一个该平台的本地指针。

*译者注：真的有必要吗？最后这一句话可能是不正确的。* 

#### Chapter 2.5.2 JVM 栈 (stack)

每个 JVM 线程都有一个私有的 JVM 栈，在该线程创建的同时被创建。
栈上存储的是栈帧 (frame) 。
这里所说的栈，和传统的编程语言（比如 C ）里所说的栈非常相似：它们都存储局部变量和表达式计算过程的中间量，并且在方法/函数的调用和返回过程发挥重要作用。
由于 JVM 栈不会被除 push & pop frames 以外的操作改变，栈帧可以在堆里构建。

*在栈这种先进后出的线性数据结构中，我们一般称向其中加入元素操作为 push, 移除其中元素为 pop.* 

对于 JVM 来说，它的线程栈在内存上的布局不必是连续的。

> 在 *The Java® Virtual Machine Specification* 第一版，Java Virtual Machine Stack 又被称作 Java Stack. 

本规范允许 JVM 栈使用定长结构或事先通过计算构造的可拓展的结构。
如果 JVM 栈有一个固定的大小，那么各线程栈的大小也可以在它们被创建时独立地设置。

> JVM 实现可以提供给程序员和用户设置 JVM 线程栈的初始大小，在其拓展改变时允许的空间大小范围。

下面列出了一些与 JVM 栈有关的异常条件：

- 如果一个线程需要使用一个超过允许大小的线程栈空间，JVM 应该抛出一个异常 `StackOverflowError`. 
- 如果一个线程栈可以动态的拓展，并且其在拓展的过程中发现剩余的内存不足以完成该拓展，或者一个新线程无法找到足够的内存来构建一个新的线程栈，JVM 应该抛出一个异常 `OutOfMemoryError`. 

*译者注：思考题 ～ 这两个 Error 定义在哪里？* 

#### Chapter 2.5.3 堆 (heap) 

JVM 在运行的过程中，有一个可供所有线程使用的全局堆。
该堆属于运行时数据区，而所有的对象、数组结构的构造所需要的内存都来自于这里。

该堆在 VM 初始化时创建。JVM 会使用一种自动化的存储策略管理系统（又称垃圾回收器）来回收不再使用的对象空间。
对象不会被显式地析构回收。
规范不会假定 JVM 使用某种垃圾回收策略，该技术可以根据实现者的系统需求自由选择。

堆本身可以是固定大小的，也可以是允许伸缩拓展的。
堆本身的内存布局不必是连续的。

下面有一个与其相关的异常：

- 如果在程序运行的过程中，内存管理系统需要的内存超过了堆能提供的能力范围，JVM 应该抛出一个异常 `OutOfMemoryError`. 

#### Chapter 2.5.4 方法区 (Method Area) 

JVM 有一个全局方法区。
这有点像进程里的 `text` 段，或者说像编程里所说的机器码。
在这里，它记录了各 `class` 的结构信息，包括类常量池 ( *run-time constant pool* ), 类成员信息 ( *field* ) ，类方法信息 ( *method* )，各方法、构造器的字节码信息，和类和接口的初始化方法。
（译者注：这里的初始化方法应该是指签名为 `<init>` 和 `<clinit>` 的特殊方法。）
( 原文：It stores per-class structures such as the run-time constant pool, field and method data, and the code for methods and constructors, including the special methods used in class and interface initialization and in instance initialization. )

方法区在虚拟机创建时被初始化。虽然方法区是堆的逻辑组成——如果你既不对它做垃圾回收，也不在上面跑某种压缩算法。
本规范不强制规定方法区所在的内存位置和在管理字节码时使用的策略。
就如堆那样，方法区既可以用定长结构实现，也可以设计成可伸缩结构。
方法区的内存不需要是连续的。

> 关于方法区的内存大小，JVM 实现也可以像「线程栈」那样提供额外的选项。

方法区相关的异常：

- 如果方法区没有足够的内存以供使用，抛出一个异常 `OutOfMemoryError`. 

#### Chapter 2.5.5 类常量池 (Run-Time Constant Pool)
