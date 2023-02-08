# Cutie 的 Java 虚拟机

## Chapter 1 简介

该虚拟机项目计划实现一个符合 SE 19 版本的 Java 虚拟机。

### Chapter 1.1 背景：Java 虚拟机

Java 虚拟机是 Java 平台的基石。
Java 平台描述了一个**硬件无关**和**操作系统**无关的世界，而这依赖于以 Java 虚拟机技术为核心的支持。
除此之外，Java 虚拟机也使得汇编代码精简，并保护用户不受（他们自己授权执行的）有害程序的伤害。

Java 虚拟机（以后统称 *JVM* ）是一个抽象的计算机，但就如一个真实的通用计算设备一样，它也有属于自己的指令集和在运行时可供操作的可记忆区域（原作「内存」，但笔者不想把它局限于仅仅是内存。）。

在历史上，JVM 的第一个原型来自于 Sun Microsystems, Inc. 公司。
它在一种像现在的 PDA (*Personal Digital Assistant*) 这样的掌上电子设备上用软件模拟的方式实现了 JVM 指令集。

而如今，Oracle 已经将 JVM 实现拓展到了移动设备，电脑，及服务器设备。
强调一下，JVM 并不预先假定自身应被何种指定的某种技术实现，或必须在某种宿主机硬件平台亦或者操作系统上实现。
JVM 的实现并不固定，实现者既可以选择将 JVM 指令再次编译到某种 CPU 的指令集来运行，也可以使用 *microcode* 甚至是直接使用支持 JVM 的 CPU. 

JVM 本身不必了解 *Java* 程序语言的细节，而只需要知道关于 `class` 文件的格式约定既可，这是一种特别的二进制协议。
一个 `class` 文件里包含了 JVM 指令（又称 *bytecodes* 字节码）和符号表用于提供额外的辅助信息。

出于安全的考量，在字节码上，JVM 要求使用其强类型语法并加之结构约束。
尽管如此，任何一门编程语言，只要其功能能够正确地被描述成一个合法的 `class` 类型文件，其代码便能被 JVM 所执行。
这样一种通用的支持性和平台无关性的特质吸引力出众，许多人在尝试实现一门新的语言时，选择使用 JVM 作为它们语言的运行平台。

而在此处描述的 JVM 与 Java SE 19 平台兼容，同时，支持在 *The Java Language Specification, Java SE 19 Edition* 所描述的 Java 语言。

### Chapter 1.2 背景：Java 程序语言

Java® 程序设计语言是一门通用目的、面向对象、支持并发的高级语言。
它的语法与 C 和 C++ 类似，但删除了很多使得 C/C++ 变得更复杂、恼人和不安全的特性。 
Java 平台为联机设备的软件开发问题而生。它被设计用于支持多种不同的宿主机架构，并允许安全地分发软件。
为了满足该条件，JVM 平台代码能够在网络间传输，并能在客户机上执行，与此同时，JVM 会向客户保证该代码的安全性。

WWW (*World Wide Web* 万维网) 的兴起，使得这些特长变得尤其有吸引力。
网页浏览器可以让数亿人能在网络中冲浪 🌊（即上网），点点手指便能访问各种多媒体内容（视频、音乐、游戏等）。
而在这过程中，无论你使用的机器是什么，它所连接的网络是什么，这都不重要——重要的是，你看到的和你听到的。

WWW 的拥趸很快便发现，通过 HTML (*HyperText Mark-up Language* 超文本标记语言) 的局限如此之大，而它的拓展 (HTML Extensions) 不仅没有解决这个问题，反而凸显了这一矛盾——例如，表单 (form) 功能。
人们发现没有任何一个浏览器能够支持所有用户想要的特性。
解决这一问题，我们不得不重新把目光聚焦在拓展性上。（原文 : Extensibility was the answer. ）

当 *HotJava* 浏览器出现的时候，其展示了关于 Java 语言 / Java 平台让人惊艳的特性，它成功地将程序 嵌入至 HTML 中。
这样一来，程序便能通过包含它的 HTML 页面被客户机下载并执行。当然，程序被下载后，仍需要经过细致的检查，以确保其的安全性，才能够被客户机执行。

正如 HTML 一样，编译好的程序也是网络、主机无关的。程序的行为不会发生变化，不论它来自于哪来，在什么样的计算设备上装载和运行。

在纳入了 Java 平台的支持后，一个网页浏览器不再受限于原来预先定义的功能范围。
用户在访问具有动态内容的网站时，不再有必要担心它们的机器会被其内容攻击。
而开发者（在 Java 的帮助下）能够一次编写，处处执行。(Write a program once, and it will run on any machine supplying a Java run-time environment. ) 

### Chapter 1.3 本规范的结构

Chapter 2: 概述了 JVM 架构。

Chapter 3: 介绍了 Java 编程语言转换为 JVM 指令的一些代码示例。

Chapter 4: 描述了 `class` 文件格式，其平台无关性的类、接口二进制表示。

Chapter 5: 定义了 JVM 启动，以及和类、接口相关的一些基本行为：装载、链接和初始化。

Chapter 6: 规范了 JVM 指令集，顺序（助记符度）展示了所有指令。

Chatper 7: JVM 指令集表（操作数序）。

在第二版的 *The Java® Virtual Machine Specification* 的第二章，有对 Java 语言的概述：Java 是被设计于支持 JVM 实现的一门语言，但并非 JVM 规范中的一部分。
*The Java Virtual Machine Specification, Java SE 19 Edition* 明确说出，读者可以参照 *The Java Language Specification, Java SE 19 Edition* 来得到 Java 语言的更多信息。

在第二版的 *The Java® Virtual Machine Specificaiton* 第八章中，其具体地列出了 JVM 线程在使用共享内存交流过程中的底层行为。
而在 *The Java Virtual Machine Specification, Java SE 19 Edition* 中，读者可以阅读 *The Java Language Specification, Java SE 19 Edition* 第 17 章，来了解关于**线程**和**锁**的更多信息。
该章充分展示了 JSR 133 Expert Group 提出的 *The Java Memory Model and Thread Specification* 机制。 

### Chapter 1.4 记号

本翻译没有特殊的记号描述。

### Chapter 1.5 反馈

Readers are invited to report technical errors and ambiguities in *The Java® Virtual Machine Specification* to jls-jvms-spec-comments@openjdk.java.net

Questions concerning the generation and manipulation of `class` files by `javac` (the reference compiler for the Java programming language ) may be sent to compiler-dev@openjdk.java.net 

## [Chpater 2](./tdoc/Chapter2.md)