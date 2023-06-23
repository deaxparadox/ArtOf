# Automatic Type Conversion

## type coercion

```js
> console.log(8 * null)
0
>
> console.log("5" - 1)
4
>
> console.log("5" + 1)
51
>
> console.log("five" * 2)
NaN
>
> console.log(false == 0)
true
> 
```

When an operator is applied to the "wrong" type of value, JavaScript will quietly convert the value to the type it needs, using a set of rules that often aren't what you want or expect. This is called *type coercion*.

[<<< Emtpy Values](107-EmtpyValues.md) ... 