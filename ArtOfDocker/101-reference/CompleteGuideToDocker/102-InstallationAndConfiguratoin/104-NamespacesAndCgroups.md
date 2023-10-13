# The Usage of Namespaes and Cgroups

    - Every container is isolated from other containers and has its own resources. These resources are called namespaces, for example, the PID (Process ID), PPID (Parent Process ID), network, mout, and others. 
    - The full isolation between containers ensure security and prohibits one container from going inside another one and messing it up.


    - Another basic security feature in Docker is cgroups, which represents the shared resources between containers, for example, the CPU, memory, and disk I/O. 
    - Docker has sensible defaults for not allowing one container to overtake the shared resources and not allowing any other container to use them.

    