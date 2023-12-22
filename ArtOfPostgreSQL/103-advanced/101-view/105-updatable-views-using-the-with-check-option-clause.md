# Creating Updatable Views Using the WITH CHECK OPTION Clause


- [<<< Materialized Views](104-materialized-view.md) |

----------

### Introduction to the WITH CHECK OPTION clause

Let's take a look at the `city` and `country` tables in the sample database.


The following statement creates an updatable view name `usa_city` that returns all cities in the United States.

```sql
CREATE VIEW usa_city AS SELECT
	city_id,
	city,
	country_id
FROM
	city
WHERE
	country_id = 103
ORDER BY
	city;
```

The following statement inserts a new row into the `city` table throught the usa_city.

```sql
INSERT INTO usa_city (city, country_id)
VALUES ('Birmingham', 102);

INSERT INTO usa_city (city, country_id)
VALUES ('Cambridge', 102);
```

The issue is that the new row inserted is not visible in the view. It may pose a secutiry issue because we may grant the permission to the users to update the cities in the United States, not the United Kingdom.

To prevent users from the insert or update a row that not visible through the view, you use the `WITH CHECK OPTION` clause when creating the view.

Let's change the `usa_city` view to include the `WITH CHECK OPTION` clause:

```sql
CREATE
OR REPLACE VIEW usa_city AS SELECT
	city_id,
	city,
	country_id
FROM
	city
WHERE
	country_id = 103
ORDER BY
	city WITH CHECK OPTION;
```

Now, run the following statement to insert another city for the `United Kingdom` country.

PostgreSQL rejected the insert and issued an error.

```
ERROR:  new row violates check option for view "usa_city"
DETAIL:  Failing row contains (604, Cambridge, 102, 2016-07-02 08:41:01.828561).
```

It works as expected.

The following statement updates the country of the city id 135 to the `United Kingdom`.

```sql
UPDATE usa_city
SET country_id = 102
WHERE
	city_id = 135;
```

PostgreSQL rejected the update and issued an error.

```
ERROR:  new row violates check option for view "usa_city"
DETAIL:  Failing row contains (135, Dallas, 102, 2016-07-02 10:37:27.466176).
```

This is because the UPDATE statement causes the row that is being update not visible through the `usa_city` view.


### The scope of check with LOCAL and CASCADED