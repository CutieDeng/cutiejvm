# Cutie JVM 

## Chapter 1. Introduction 

The Java® Programming Language is a general-purpose, concurrent, object-oriented language. 

The Java platform was initially developed to address the problems of building software for networked consumer devices. It was deisgned to support multiple host architectures and to allow secure delivery of software components. To meet these requirements, compiled code had to survive transport across networks, operate on any client, and assure the client that it was safe to run. 

## Chapter 2. Java Virtual Machine 

The Java Virtual Machine is the cornerstone of the Java platform. It is the component of the technology responsible for its hardware- and operating system-independence, the small size of its complied code, and its ability to protect users from malicious programs. 

The Java Virtual Machine is an abstract computing machine. Like a real computing machine, it has an instruction set and manipulates various memory areas at run time. 

Specially, the Java Virtual Machine knows nothing of the Java programming language, only of a particular binary format, the `class` file format. A `class` file contains Java Virtual Machine instructions ( or *bytecodes* ) and a symbol table, as well as other ancillary information. 

