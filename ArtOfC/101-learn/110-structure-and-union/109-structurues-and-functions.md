# Structures and Functions

Three methods by which the values of a structure can be transffered from on function to another.

1. The first method is to pass each member of the structure as an actual argument of the function call. The actual arguments are then treated independently like ordinary variables. This is the most element method and become unmanageable and inefficient when t he structure size is large.
2. The second method involves passing of a copy of entire structure to the called function. Since the function is working on a copy of structure, any changes to structure menber within the function are not reflected i nthe original structure (in the calling function). It is, therefore necessary for the function to return the entire structure back to the calling function. All compilers may not suuport this method of passing the entire structure as a parameter.
3. The third approach employs a concept called *pointers* to pass the structure as an argument. In this case, the address location of the structure is passed to the called function. The function can access indrectly the entire structure and work on it. This is similar to the ways arrays are passed to function. This method is more efficient as compare to the second one.

----------

The genral format for sending a copy of a structure to the called function is:

```c
function_name(structure_variable_name);
```

The called functiontakes the following form:


```
data_type  function_name(struct_type st_name) 
{
    ......
    ......
    return(expression);
}
```