# Basic Layout


A consumer is a subclass of either `channels.consumer.AsyncConsumer` or `channels.consumer.SyncConsumer`. As these names suggest, one will expect you to write async-capable code, while the other will run your code synchronously in a threadpool.

#### SyncConsumer:

Let's look at a basic example of a `SyncConsumer`:

```py
from channels.consumer import SyncConsumer

class EchoConsumer(SyncConsumer):

    def websocket_connect(self, event):
        self.send({
            "type": "websocket.accept",
        })

    def websocket_receive(self, event):
        self.send({
            "type": "websocket.send",
            "text": event["text"],
        })
```

This is a very simple WebSocket echo server - it will accept all incoming WebSocket connections, and then reply to all incoming WebSocket text frames with the same text.

Consumers are structured around a series of named methods corresponding to the `type` value of the messages they are going to receive, with any `.` replaced by `_`. The two handlers above are handling `websocket.connect` and `websocket.receive` messages respectively.

Apart from that, the only other basic API is `self.send(event)`. This lets you send events back to the client or protocol server as defined by the protocol - if you read the WebSocket protocol, you’ll see that the dict we send above is how you send a text frame to the client.

## AsyncConsumer

The `AsyncConsumer` is laid out very similarly, but all the handler methods must be coroutines and `self.send` is a coroutine.

```py
from channels.consumer import AsyncConsumer

class EchoConsumer(AsyncConsumer):

    async def websocket_connect(self, event):
        await self.send({
            "type": "websocket.accept",
        })

    async def websocket_receive(self, event):
        await self.send({
            "type": "websocket.send",
            "text": event["text"],
        })
```