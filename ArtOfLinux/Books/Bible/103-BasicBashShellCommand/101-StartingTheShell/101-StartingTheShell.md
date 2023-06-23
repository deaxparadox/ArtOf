# Starting the Shell

- The GUN bash shell is a program that provides interactive access to the Linux system. It runs as a regular program and normally started whenever a user logs in to a terminal. The shell that the system starts depends on your user ID configuration.


## /etc/passwd

- The `/etc/passwd` file contains a list of all the system user accounts, along with some basic configuration information about each user.  

```bash
dnsmasq:x:109:65534:dnsmasq,,,:/var/lib/misc:/usr/sbin/nologin
frost:x:1001:1001::/home/frost:/usr/bin/bash
```

- Each entry has seven data fields, with fields separated by colons. The system uses the data in the fields to assign specific features for the user.


