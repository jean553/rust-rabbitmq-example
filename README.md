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
