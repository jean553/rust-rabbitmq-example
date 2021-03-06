# rust-rabbitmq-example

Simple Rust RabbitMQ usage example.

## Table of content
 - [Start the project](#start-the-project)
 - [Connect to the container](#connect-to-the-container)
 - [Build the project](#build-the-project)
 - [Run the project](#run-the-project)
 - [Enable management HTTP API](#enable-management-http-api)
 - [Patterns](#patterns)
    * [Simple queue](#simple-queue)
    * [Competing consumers](#competing-consumers)
    * [Message consumed acknowledgement](#message-consumed-acknowledgement)
    * [Message durability](#message-durability)
    * [Prefetch count](#prefetch-count)
 - [Exchanges](#exchanges)
    * [Fanout](#fanout)
    * [Direct](#direct)
    * [Topics](#topics)
 - [Remote Procedure Call](#remote-procedure-call)
 - [Delete all queue messages](#delete-all-queue-messages)
 - [Sources](#sources)

## Start the project

```sh
vagrant up
```

## Connect to the container

```sh
vagrant ssh
```

## Build the project

```sh
cargo build --release
```

## Run the project

```sh
./target/release/example [OPTIONS]
```

Available options:
 * `--consumers` - amount of consumers threads (ex: `--consumers 5`)
 * `--enable-ack` - enable consumed messages acknowledgement (removed from the queue only if explicitely aknowledged by a consumer)
 * `--durable` - enable queued messages durability (copied on disk if the queue stops)
 * `--help` - display help details

## Enable management HTTP API

A HTTP API is available on RabbitMQ in order to monitor the queue.
This feature is not enabled by default. Follow the following steps to enable it.

Connect to the RabbitMQ container:

```sh
docker exec -it rust_rabbitmq_example_queue /bin/bash
```

Enable the HTTP API:

```sh
rabbitmq-plugins enable rabbitmq_management
```

Outside of the container, check the API is working:

```sh
curl -u guest:guest http://localhost:8080/api/vhosts
```

The authorization header must contain a Basic Auth token,
representing the base64 digest of "guest:guest",
so we add an `-u` value.

## Patterns

### Simple queue

A simple queue simply contains a single producer and a single consumer.
The producer pushes messages into the queue while the consumer get messages from the queue.

To run a simple queue, simply run the tool without any option. This mode is the default one.

```sh
./target/release/example << EOF
push a
push b
push c
EOF
```

Each message is consumed one by one.

```sh
[Consumer 0] Started.
[Consumer 0] Start handling message: a
[Consumer 0] Terminate handling message: a
[Consumer 0] Start handling message: b
[Consumer 0] Terminate handling message: b
[Consumer 0] Start handling message: c
[Consumer 0] Terminate handling message: c
```

### Competing consumers

A single producer, one queue and multiple consumers (also called "workers").
The producer pushes messages into the queue while the consumers get messages from the queue.
Using this tool, the amount of consumers is an amount of internal threads that are started.

To run competiting consumers, run the command with an option indicating the amount of consumers:

```sh
./target/release/example --consumers 3 << EOF
push a
push b
push c
EOF
```

Each message is consumed at the same time, one by consumer.

```sh
[Consumer 0] Started.
[Consumer 1] Started.
[Consumer 2] Started.
[Consumer 2] Start handling message: a
[Consumer 0] Start handling message: b
[Consumer 1] Start handling message: c
[Consumer 1] Terminate handling message: c
[Consumer 2] Terminate handling message: a
[Consumer 0] Terminate handling message: b
```

### Message consumed acknowledgement

Consumed messages are kept within the queue as long as the queue does not get an aknowledgement
from the consumer once the message has been consumed.

Without aknowledgement, the message is the re-queued after being consumed.
The message will be re-consumed when new consumers starts.

In order to enable aknowledgement, use the `enable-ack` option:

```sh
./target/release/example --consumers 2 --enable-ack true
```

Without acknowledgement enabled, the message is consumed everytime a new consumer is connected:

```sh
./target/release/example
[Consumer 0] Started.
> push a
[Consumer 0] Start handling message: a
[Consumer 0] End handling message: a

./target/release/example
[Consumer 0] Started.
[Consumer 0] Start handling message: a
[Consumer 0] End handling message: a
```

This does not happen when aknowledgement is enabled:

```sh
./target/release/example --enable-ack true
[Consumer 0] Started.
> push a
[Consumer 0] Start handling message: a
[Consumer 0] End handling message: a

./target/release/example
[Consumer 0] Started.
```

This can be verified using the management HTTP API:

```sh
curl -i -u guest:guest http://localhost:8080/api/vhosts
```

If acknowledgement is enabled, `messages_stats/messages` is equal to 0 when all messages have been consumed.
If acknowledgement is not enabled, `messages_stats/messages` is equal to the pushed amount of messages,
no matter if they have been consumed or not.

### Message durability

When message durability is enabled, if the queue stops (because of failures or other),
then the messages that have not been consumed are written on disk.

They are then re-queued again if the queue is restarted.

In order to enable messages durability:

```sh
./target/release/example --durable true
```

### Prefetch count

Indicates how many unknowledged messages one consumer can consumes until it refuses new messages.

```sh
./target/release/example --prefetch-count 2 --enable-ack true --consumers 2
```

## Exchanges

An exchange is setup between the producer and the queue(s).
It is used to forward message to one specific queue or to all queues.

### Fanout

The producer sends messages to the fanout exchange.
The fanout exchange forwards every message to all queues.
The relationship between the exchange and one queue is called a binding.

### Direct

Messages are routed to one queue according to their content
(for instance, all messages with the *high* severity are sent into the first queue,
all the others are sent into the second queue).

This is allowed to bind multiple queues with the same binding key.

### Topics

Topic exchanges are like direct exchanges, but one queue can be bind to multiple words.
For instance, it is possible to redirect all the messages with the pattern `*_some_message_*`
(`*` means "one word") to one specific queue.

## Remote Procedure Call

In that kind of schema, there are usually two queues:
 * one queue to redirect messages to the server,
 * one queue to send back messages to the client (created by the client when it connects to the server)

A `correlation id` is used to know which message from the callback queue is linked to what message that has been sent to the server queue.

## Delete all queue messages

In order to remove all the messages from the queue,
connect into the queue container and execute the following command:

```sh
rabbitmqctl purge_queue example_queue
```

## Sources

RabbitMQ tutorials (https://www.rabbitmq.com/tutorials)
