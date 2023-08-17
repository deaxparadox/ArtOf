## `#`  Comments

Lines begining with a `#` (with the exception of `#!`) are comments and will not be executed:

```bash
# This line is a comment.
```

Comments may also occur following the end of a command:

```bash
$ echo "A comment will follow."    # Comment here.
A comment will follow.
```

Comments may also follow whitespace at the begining of a line.

Comments may even be embedded wihin a pipe.

```bash
initial=( `cat "$startfile" | sed -e '/#/d' | tr -d '\n' |\
# Delete lines containing '#' comment character.
sed -e 's/\./\. /g' -e 's/_/_ /g'` )
# Excerpted from life.sh script
```

A quoted or an escaped `#` in an echo statement does *not* begin a commment.

```bash
echo "The # here does not begin a comment."
echo 'The # here does not begin a comment.'
```

We can also use backlash to escape `#` so it will not act as command and with out backlash it will act as comment:

```bash
echo The \# here does not begin a comment.
echo The # here begins a comment.
```


A `#` appears in certain parameter-substitution constructs and in numerical constant expression:

```bash
echo ${PATH#*:}         # Parameter substitution, not a comment.
echo $(( 2#101011 ))    # Base conversion, not a comment.

```
1
## `;`  Command separator

Permits putting two or more commands on the same line.

```bash
$ echo hello; echo there
hello
there
```

## `;;` Terminator in a `case` option [double semicolon]

Let's build a small script for understanting this terminator:

```bash
#!/usr/bin/env bash

variable="abc"

case "$variable" in 
    abc) echo "\$variable = abc" ;;
    xyx) echo "\$variable = xyz" ;;
esac
```

- output

```bash
$variable = abc
```

## `;;&`, `;&` 

Terminators in a case option (version 4+  of bash)


## `dots` [period]

#### dot command

Equivalent to source command. This is a bash builtin feature. Create a file **test.sh**, then add the executing permission to the file **chmod +x test.sh**, now we can run this file using *period* or *source command*

```bash
$ touch test.sh             # file created
$ vim test.sh               # open the file and add `echo "Hello everyone"`
$ chmod +x test.sh          # adding executable permission
$ ./test.sh                 # output: Hello everyone
$ # or
$ source test.sh 
```

#### `dot` as a component of a filename:

