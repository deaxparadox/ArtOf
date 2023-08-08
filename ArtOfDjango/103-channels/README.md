# Django Channels

Channels is a project that takes Django and extends its abilities beyond HTTP - to handle WebSockets, chat protocols, IoT protocols, and more. It’s built on a Python specification called ASGI.

Channels builds upon the native ASGI support in Django. Whilst Django still handles traditional HTTP, Channels gives you the choice to handle other connections in either a synchronous or asynchronous style.

Channels is compromised of several packages:

- **Channels**, the Django integration layer
- **Daphne**, the HTTP and Websocket termination layer
- **asgiref**, the base ASGI library
- **channels_redis**, the Redis channels layer backend

Channels also bundles this event-driven architecture with *channel layers*, a system that allows you to easily communicate between processes, and separate your project into different processes.

##  Scopes and Event 

Channels and ASGI split up incoming connections into two components: a *scope*, and a series of *events*.

The *scope* is a set of details about a single incoming connection - such as the path  a web request was made from, or the originating IP address of a WebSocket, or the user messaging a chatbot. The scope persists throughtout the connection.

- For HTTP, the scope just lasts a single request. 
- For WebSockets, it lasts for the lifetime of the socket (but changes if the socket closes and reconnects). 
- For other protocols, it varies based on how the protocol’s ASGI spec is written; for example, it’s likely that a chatbot protocol would keep one scope open for the entirety of a user’s conversation with the bot, even if the underlying chat protocol is stateless.

During the lifetime of this *scope*, a series of events occur. These represent user interactions - making a HTTP request, for example, or sending a WebSocket frame. Your Channels or ASGI applications will be **instantiated once per scope**, and then be fed the stream of events happening within that scope to decide what action to take.

#### An example with HTTP:

- The user makes an HTTP request.
- We open up a new `http` type scope with details of the request’s path, method, headers, etc.
- We send a `http.request` event with the HTTP body content
- The Channels or ASGI application processes this and generates a `http.response` event to send back to the browser and close the connection.
- The HTTP request/response is completed and the scope is destroyed.

#### An example with a chatbot:

- The user sends a first message to the chatbot.
- This opens a scope containing the user’s username, chosen name, and user ID.
- The application is given a `chat.received_message` event with the event text. It does not have to respond, but could send one, two or more other chat messages back as `chat.send_message` events if it wanted to.
- The user sends more messages to the chatbot and more `chat.received_message` events are generated.
- After a timeout or when the application process is restarted the scope is closed.

## What is a consumer?

A consumer is the basic unit of Channels code. We call it a *consumer* as it *consumes events*, but you can think of it as its own tiny little application. When a request or new socket comes in, Channels will follow its routing table find the right consumer for that incoming connection, and start up a copy of it.

Consumers are long-running. They can also be short-running - after all, HTTP requests can also be served by consumers - but they’re built around the idea of living for a little while (they live for the duration of a scope).

```py
class ChatConsumer(WebsocketConsumer):

    def connect(self):
        self.username = "Anonymous"
        self.accept()
        self.send(text_data="[Welcome %s!]" % self.username)

    def receive(self, *, text_data):
        if text_data.startswith("/name"):
            self.username = text_data[5:].strip()
            self.send(text_data="[set your username to %s]" % self.username)
        else:
            self.send(text_data=self.username + ": " + text_data)

    def disconnect(self, message):
        pass
```

Each different protocol has different kinds of events that happen, and each type is represented by a different method. You write code that handles each event, and Channels will take care of scheduling them and running them all in parallel.

Underneath, Channels is running on a fully asynchronous event loop, and if you write code like above, it will get called in a synchronous thread. This means you can safely do blocking operations, like calling the Django ORM:

```py
class LogConsumer(WebsocketConsumer):

    def connect(self, message):
        Log.objects.create(
            type="connected",
            client=self.scope["client"],
        )
```

However, if you want more control and you’re willing to work only in asynchronous functions, you can write fully asynchronous consumers:

```py
class PingConsumer(AsyncConsumer):
    async def websocket_connect(self, message):
        await self.send({
            "type": "websocket.accept",
        })

    async def websocket_receive(self, message):
        await asyncio.sleep(1)
        await self.send({
            "type": "websocket.send",
            "text": "pong",
        })
```

## Routing and Multiple Protocols

You can combine multiple consumers (which are, their own ASGI apps) into one bigger app that represents your project using routing:

```py
application = URLRouter([
    path("chat/admin/", AdminChatConsumer.as_asgi()),
    path("chat/", PublicChatConsumer.as_asgi()),
])
```

Channels is not just built around the world of HTTP and WebSockets - it also allows you to build any protocol into a Django environment, by building a server that maps those protocols into a similar set of events. For example, you can build a chatbot in a similar style:


```py
class ChattyBotConsumer(SyncConsumer):

    def telegram_message(self, message):
        """
        Simple echo handler for telegram messages in any chat.
        """
        self.send({
            "type": "telegram.message",
            "text": "You said: %s" % message["text"],
        })
```

- And then use another router to have the one project able to serve both WebSockets and chat requests:

```py
application = ProtocolTypeRouter({

    "websocket": URLRouter([
        path("chat/admin/", AdminChatConsumer.as_asgi()),
        path("chat/", PublicChatConsumer.as_asgi()),
    ]),

    "telegram": ChattyBotConsumer.as_asgi(),
})
```


## Cross-Process Communication

Much like a standard WSGI server, your application code that is handling protocol events runs inside the server process itself - for example, WebSocket handling code runs inside your WebSocket server process.

Each socket or connection to your overall application is handled by an *application instance* inside one of these servers. They get called and can send data back to the client directly.

However, as you build more complex application systems you start needing to communicate between different *application instances* - for example, if you are building a chatroom, when one *application instance* receives an incoming message, it needs to distribute it out to any other instances that represent people in the chatroom.

You can do this by polling a database, but Channels introduces the idea of a *channel layer*, a low-level abstraction around a set of transports that allow you to send information between different processes. Each application instance has a unique *channel name*, and can join *groups*, allowing both point-to-point and broadcast messaging.

##### Channel layers are an optional part of Channels, and can be disabled if you want (by setting the CHANNEL_LAYERS setting to an empty value).

```py
# In a consumer
self.channel_layer.send(
    'event',
    {
        'type': 'message',
        'channel': channel,
        'text': text,
    }
)
```

You can also send messages to a dedicated process that’s listening on its own, fixed channel name:

```py
# In a consumer
self.channel_layer.send(
    "myproject.thumbnail_notifications",
    {
        "type": "thumbnail.generate",
        "id": 90902949,
    },
)
```


## Django Integration

Channels ships with easy drop-in support for common Django features, like sessions and authentication. You can combine authentication with your WebSocket views by just adding the right middleware around them:

```py
from django.core.asgi import get_asgi_application
from django.urls import re_path

# Initialize Django ASGI application early to ensure the AppRegistry
# is populated before importing code that may import ORM models.
django_asgi_app = get_asgi_application()

from channels.routing import ProtocolTypeRouter, URLRouter
from channels.auth import AuthMiddlewareStack
from channels.security.websocket import AllowedHostsOriginValidator

application = ProtocolTypeRouter({
    "http": django_asgi_app,
    "websocket": AllowedHostsOriginValidator(
        AuthMiddlewareStack(
            URLRouter([
                re_path(r"^front(end)/$", consumers.AsyncChatConsumer.as_asgi()),
            ])
        )
    ),
})
```