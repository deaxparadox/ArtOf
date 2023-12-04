# Drop View

### Introduction to PostgreSQL DROP VIEW statement

The `DROP VIEW` statement removes a view from the database. The following illustrates the syntax of the `DROP VIEW` statement;

```sql
DROP VIEW [IF EXISTS] view_name
[CASCADE | RESTRICT]
```

In this syntax:

- First, specify the name of the view after the `DROP VIEW` keywords.
- Second, use the `IF EXISTS` option to drop a view only it it exists. If you don't use the `IF EXISTS` option and drop a view that does not exists, PostgreSQL will issue an error. However, if you use the `IF EXISTS` option, PostgreSQL issues a notice instead.
- Third, use the `RESTRICT` option to reject the removal of the view if there are any objects depending on it. The `RESTRICT` option is the default. If you use the `CASCADE` option, the `DROP VIEW` automatically drops objects that depend on view and all objects that depend on those objects.

To remove multiple views using a single statement, you specify a comma separated list of view name after the `DROP VIEW` keywords like this:

```sql
DROP VIEW [IF EXISTS] view_name1, view_name2, ...;
```

To execute the `DROP VIEW` statement, you must be the owner of the view.

