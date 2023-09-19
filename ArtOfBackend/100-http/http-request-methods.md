# HTTP request methods

HTTP defines a set of **request methods** to indicate the desired action to be performed for a given resource. Although they can also be nouns, these request methods are sometimes referred to as HTTP verbs. Each of them implements a different semantic, but some common features are shared by a group of them: e.g. a request method can be safe, idempotent, or cacheable.

### GET 

The `GET` method requests a representatino of the specified resource. Requests using `GET` should only retrieve data.


```http
GET /index.html
```

### HEAD

The `HEAD` method asks for a response identical to a `GET` request, but without the response body.

The HTTP `HEAD` method requests the `headers` that would be returned if the `HEAD` request's URL was instead requested with the HTTP `GET` method. For example, if a URL might produce a large download, a HEAD request could read its `Content-Length` header to check the filesize without actually downloading the file.

If the response to a `HEAD` request shows that a cached URL response is now outdated, the cached copy is invalidated even if no `GET` request was made.

```http
HEAD /index.html
```

### POST

The `POST` method submits an entity to the specified resource, often causing a change in state or side effects on the server.


The HTTP `POST` method sends data to the server. The type of the body of the request is indicated by the `Content-Type` header.

The difference between `PUT` and `POST` is that `PUT` is idempotent: calling it once or several times successively has the same effect (that is no side effect), where successive identical `POST` may have additional effects, like passing an order several times.

A `POST` request is typically sent via an `HTML form` and results in a change on the server. In this case, the content type is selected by putting the adequate string in the enctype attribute of the `<form>` element or the `formenctype` attribute of the `<input>` or `<button>` elements: