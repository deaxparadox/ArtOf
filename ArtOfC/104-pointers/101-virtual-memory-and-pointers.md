# Virtual memory and pointers

The pointer addresses displayed on a *virtual operating system* are not likely to be the real physical memory addresses. A virutal operating system allows a program to be split across the machine's physical address space.

An application is split into pages/frames. These pages represent areas of main memory. The pages of the application are allocated to different, potentially noncontiguous areas of memory and may not all be in memory at the same time. If the operating system needs memory currently held by a page, the memory may be swapped out to secondary storage and then reload at a later time, frequently at a differnt memory location. These capabilities provides a virtual operating system with considerable flexibility in how it manages memory.

Each program assumes it has access to the machines' entire physical memory space. In reality it does not. The address used by a program in a virtual adderss. The operating system maps the virtual address to a real physical memory address when needed.

This means code and data in a page may be in different physical locations as the program executes. The application's virutal address do not charge; they are the addresses we see when we examine the contents of a pointer. The virtual addresses are transparently mapped to real addresses by the operating system.

The operating system handles all of this, and it is not something that the programmer has control over or needs to worry about. Understanding these issues  explains the addresses returned by a program running in a virtual operating system.

[<<< README](README.md) .. [Deferencing a Pointer Using the Indirection Operator >>>](102-dereferencing-a-pointer-using-the-indirection-operator.md)