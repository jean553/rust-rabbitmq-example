# rust-rabbitmq-example

Simple Rust RabbitMQ usage example.

## Table of content
 - [Start the project](#start-the-project)
 - [Connect to the container](#connect-to-the-container)
 - [Build the project](#build-the-project)
 - [Run the project](#run-the-project)
 - [Patterns](#patterns)
    * [Simple queue](#simple-queue)
    * [Competing consumers](#competing-consumers)
    * [Message consumed acknowledgement](#message-consumed-acknowledgement)
    * [Message durability](#message-durability)

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

## Patterns

### Simple queue

A simple queue simply contains a single producer and a single consumer.
The producer pushes messages into the queue while the consumer get messages from the queue.

To run a simple queue, simply run the tool without any option. This mode is the default one.

```sh
./target/release/example
```

### Competing consumers

A single producer, one queue and multiple consumers (also called "workers").
The producer pushes messages into the queue while the consumers get messages from the queue.
Using this tool, the amount of consumers is an amount of internal threads that are started.

To run competiting consumers, run the command with an option indicating the amount of consumers:

```sh
./target/release/example --consumers 2
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

### Message durability

When message durability is enabled, if the queue stops (because of failures or other),
then the messages that have not been consumed are written on disk.

They are then re-queued again if the queue is restarted.

In order to enable messages durability:

```sh
./target/release/example --durable true
```
