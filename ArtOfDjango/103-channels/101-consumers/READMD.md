# Consumers

Channels provides you with Consumers, a rich abstraction that allows you to create ASGI application easily.

Consumers do a couple of things particular

- Structure your code as a series of functions to be called whenever an event happens, rather than making you write an event loop.
- Allow you to write synchronous or async code, and deal with handoffs and threading for you.

Of course, you are free to ignore consumers and use the other parts of Channels - like routing, session handling and authentication - with any ASGI app, but they’re generally the best way to write your application code.