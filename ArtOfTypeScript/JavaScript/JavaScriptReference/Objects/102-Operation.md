# Operation on property

### Property values are accesible using the dot notation:

```js
> console.log(account.name)
EveryWhereLinux
>
> console.log(account.os)
linux
> 
```

### Adding property

#### `.` operator

```js
> account.os_name = "debian"
"debian"
>
> account
{ name: "EveryWhereLinux", os: "linux", os_name: "debian" }
> 
```

#### another way to add property to the object is using `square bracket and qutoes` such as `object["property"] = value`, using this way you can also add property that contain space:

```js
> account["first user"] = "frost";
"frost"
>
> account
{
  name: "EveryWhereLinux",
  os: "linux",
  os_name: "debian",
  "first user": "frost"
}
> 
```

### Removing property from object

#### Removing property using operator:

```js
> delete account.os_name
true
>
> account
{ name: "EveryWhereLinux", os: "linux", "first user": "frost" }
>
```

#### To remove that contain space, or any property

```js
> delete account["first user"];
true
> 
> account
{ name: "EveryWhereLinux", os: "linux" }
>
```