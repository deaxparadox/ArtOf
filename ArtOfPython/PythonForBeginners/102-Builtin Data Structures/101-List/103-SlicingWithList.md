# Slicing with list


## Slicing

You use slicing to create a substring, we have already seen slicing in `string`.

Let's move on to example:

Accessing sequence from 3 with index `2` to 5 with index `4`, last element is not included:

```py
>>> a[2:5]
[3, 4, 5]
>>> 
```

You can check the index of 5:

```py
>>> a.index(5)
4
>>> 
```

So, you can element with `index 5` is not included in substring.

To get full list:


```py
>>> a[:]
[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
>>> 
```


[`tuple`](104-Tuple.md)