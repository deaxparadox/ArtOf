# JavaScript Fundamentals: Hello World!

There are may way to run JavaScript, such as `nodejs`, `deno` and `bun` all of these three are runtimes. But for now we are going to stick with Browser-specifc commands, using `script` tag in html page.

### The "script" tag

JavaScript programs can be inserted almost anywhere into an HTML document using the `<script>` tag.

For instance:

```html
<!DOCTYPE HTML>
<html>

<body>

  <p>Before the script...</p>

  <script>
    alert( 'Hello, world!' );
  </script>

  <p>...After the script.</p>

</body>

</html>
```

The `<script>` tag contains JavaScript code which is automatically executed when the browser processes the tag.


### External Scripts

If we have a lot of JavaScript code, we can put it into a separate file.

Script files are attached to HTML with the `src` attributes:


```html
<script src="/path/to/script.js"></script>
```

Here, `/path/to/script.js` is an absolute path to the script from the site root. One can also provide a relative path from the current page. For instance, `src="script.js"`, just like `src="./script.js"`, would mean a file `"script.js"` in the current folder.

We can give a full URL as well. For instance:

```html
<script src="https://cdnjs.cloudflare.com/ajax/libs/lodash.js/4.17.11/lodash.js"></script>
```

To attach several scripts, use multiple tags:

```html
<script src="/js/script1.js"></script>
<script src="/js/script2.js"></script>
```

----------

# Note

As a rule, only the simplest scripts are put into HTML. More complex ones reside in separate files.

The benefit of a separate file is that the browser will download it and store it in its cache.

Other pages that reference the same script will take it from the cache instead of downloading it, so the file is actually downloaded only once.

That reduces traffic and makes pages faster.

----------

If `src` is set, the script content is ignored

A single `<script>` tag can't have both the `src` attribute and code inside.

This won't work:

```html
<script src="file.js">
  alert(1); // the content is ignored, because src is set
</script>
```

We must choose either an external scirpt or a regular scirpt with code.

The example above can be split into two scripts to work:

```html
<script src="file.js"></script>
<script>
  alert(1);
</script>
```