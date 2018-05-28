# rust-rabbitmq-example

Simple Rust RabbitMQ usage example.

## Table of content
 - [Start the project](#start-the-project)
 - [Connect to the container](#connect-to-the-container)
 - [Build the project](#build-the-project)
 - [Run the project](#run-the-project)
 - [Patterns](#patterns)
    * [Simple queue](#simple-queue)

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
 * `--help` - display help details

## Patterns

### Simple queue

A simple queue simply contains a single producer and a single consumer.
The producer pushes messages into the queue while the consumer get messages from the queue.

To run a simple queue, simply run the tool without any option. This mode is the default one.

```sh
./target/release/example
```
