# Looking at the Linux filesystem

- Linux stores files within a single directory structure, called a *virtual memory*.

- The virutal directory contains the paths from all the storage devices installed on the computer, merged into a single directory structure.

- The Linux virtual directory structure contains a single base directory, called the root.

- Directories and files beneath the root directory are listed based on the directory path used to get to them.

- Linux uses a forward slash (/) to denote directories in file paths.

```bash
/home/Rich/Documents/test.doc
```

- This indicates the file *text.doc* in the directory *Documents*, under the directory *rich*, which contained in the directory *home*.


The tricky part about the Linux virtual directory is how it incorporates each storage device.

- The first hard drive installed in a Linux system is called the *root drive*.
- The root drive contains the virutal directory core. 
- Everything else builds from there.

On the root drive, Linux can use special directories as *mount points*.

- Mount points are directories in the virutal directory where you can assign additional storage devices.
- Linux causes files and directories to appear within these mount point directories, even though they are physically stored on a different drive.


## Common Linux Directory Names

- **/**: root of the virtual directory, where normally, no files are placed.
- **/bin**: binary directory, where many GNU user-level utilities are stored.
- **/boot**: boot directory, where boot files are stored.
- **/dev**: device directory, where LInux creates device nodes.
- **/etc**: system configuration files directory
- **/home**: home directory, where Linux creates user directories
- **/lib**:  library directory, where system and application library files are stored.
- **/media**: media directory, a common place for mount points used for removable media
- **/mnt**: mount directory, another common place for mount points used for removable media
- **/opt**: optional directory, often used to stored third-party software packages and data files.
- **/proc**: process directory, where current hardware and process information is stored.
- **/root**: root home directory
- **/sbin**: root home directory
- **/run**: run direcotry, where runtime data is held during system operation
- **/srv**: service directory, where local services store their files.
- **/sys**: system directory, where system hardware information files are stored.
- **/tmp**: temporary directory, where temporary work files can be created and destroyed.
- **/use**: user binary directory, where the bulk of GNU user-level utilities and data files are stored.
- **/var**: variable directory, for files that changed frequently, such as log files.

The common Linux directory names based up the Filesystem Hierarchy Standard (FHS). 

When you log in to your system and reach a shell CLI prompt, your session starts in your home directory. You directory is a unique directory assigned to your user account. When a user account is created, the system normally assigns a unique directory for the account.
