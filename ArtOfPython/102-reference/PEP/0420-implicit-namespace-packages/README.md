# PEP 420 - Implicit Namespace Packages

Namespace packages are a mechanism for splitting a single Python package across multiple directories on disk. In current Python versions, an algorithm to compute the packages `__path__` must be formulated. With the enhancement proposed here, the import machinery itself will construct the list of directories that make up the package. 

### Terminology 

Within this PEP:

- “package” refers to Python packages as defined by Python’s import statement.
- “distribution” refers to separately installable sets of Python modules as stored in the Python package index, and installed by distutils or setuptools.
- “vendor package” refers to groups of files installed by an operating system’s packaging mechanism (e.g. Debian or Redhat packages install on Linux systems).
- “regular package” refers to packages as they are implemented in Python 3.2 and earlier.
- “portion” refers to a set of files in a single directory (possibly stored in a zip file) that contribute to a namespace package.
- “legacy portion” refers to a portion that uses `__path__` manipulation in order to implement namespace packages.

The PEP defines a new type of packages, the "namespace package".