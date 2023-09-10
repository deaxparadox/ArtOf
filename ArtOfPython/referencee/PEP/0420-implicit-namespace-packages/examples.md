# Examples

### Nested namespace packages

The example uses the following directory structure:

```
Lib/test/namespace_pkgs
    project1
        parent
            child
                one.py
    project2
        parent
            child
                two.py
```

Here, both parent and child are namespace packages: Portions of them  exist in different directories, and they do not have `__init__.py` files.

Here we add the parent directories to sys.path, and show that the portions are correctly found:

```py
>>> import sys
>>> sys.path += ['Lib/test/namespace_pkgs/project1', 'Lib/test/namespace_pkgs/project2']
>>> import parent.child.one
>>> parent.__path__
_NamespacePath(['Lib/test/namespace_pkgs/project1/parent', 'Lib/test/namespace_pkgs/project2/parent'])
>>> parent.child.__path__
_NamespacePath(['Lib/test/namespace_pkgs/project1/parent/child', 'Lib/test/namespace_pkgs/project2/parent/child'])
>>> import parent.child.two
>>>
```

### Dynamic path computation

This example uses a similar directory structure, but adds a third portion:

```
Lib/test/namespace_pkgs
    project1
        parent
            child
                one.py
    project2
        parent
            child
                two.py
    project3
        parent
            child
                three.py
```

We add `project1` and `project2` to `sys.path`, then import `parent.child.one` and `parent.child.two`. Then we add the `project3` to `sys.path` and when `parent.child.three` is imported, `project3/parent` is automatically added to `parent.__path__`:

```py
# add the first two parent paths to sys.path
>>> import sys
>>> sys.path += ['Lib/test/namespace_pkgs/project1', 'Lib/test/namespace_pkgs/project2']

# parent.child.one can be imported, because project1 was added to sys.path:
>>> import parent.child.one
>>> parent.__path__
_NamespacePath(['Lib/test/namespace_pkgs/project1/parent', 'Lib/test/namespace_pkgs/project2/parent'])

# parent.child.__path__ contains project1/parent/child and project2/parent/child, but not project3/parent/child:
>>> parent.child.__path__
_NamespacePath(['Lib/test/namespace_pkgs/project1/parent/child', 'Lib/test/namespace_pkgs/project2/parent/child'])

# parent.child.two can be imported, because project2 was added to sys.path:
>>> import parent.child.two

# we cannot import parent.child.three, because project3 is not in the path:
>>> import parent.child.three
Traceback (most recent call last):
  File "<stdin>", line 1, in <module>
  File "<frozen importlib._bootstrap>", line 1286, in _find_and_load
  File "<frozen importlib._bootstrap>", line 1250, in _find_and_load_unlocked
ImportError: No module named 'parent.child.three'

# now add project3 to sys.path:
>>> sys.path.append('Lib/test/namespace_pkgs/project3')

# and now parent.child.three can be imported:
>>> import parent.child.three

# project3/parent has been added to parent.__path__:
>>> parent.__path__
_NamespacePath(['Lib/test/namespace_pkgs/project1/parent', 'Lib/test/namespace_pkgs/project2/parent', 'Lib/test/namespace_pkgs/project3/parent'])

# and project3/parent/child has been added to parent.child.__path__
>>> parent.child.__path__
_NamespacePath(['Lib/test/namespace_pkgs/project1/parent/child', 'Lib/test/namespace_pkgs/project2/parent/child', 'Lib/test/namespace_pkgs/project3/parent/child'])
>>>
```