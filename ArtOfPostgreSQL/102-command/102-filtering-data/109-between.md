# PostgreSQL BETWEEN

You use the `BETWEEN` operator to match a value against a range of values. The following illustrates the syntax of the `BETWEEN` operator:

```
value BETWEEN low AND high;
```

If the `value` is greater than or equal to the `low` value and less than or equal to the `hight` value, the expression returns true, otherwise, it returns false.

You can write the `BETWEEN` operator by using the greater than or equal ( `>=` ) or less than or equal ( `<=` ) operators like this:

```
value >= low and value <= high
```