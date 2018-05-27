extern crate amqp;
extern crate clap;

use amqp::{
    Session,
    Channel,
    Table,
    Basic,
};

use amqp::protocol::queue::DeclareOk;

use amqp::protocol::basic::{
    BasicProperties,
    Deliver,
};

use clap::App;

use std::thread::spawn;
use std::io::stdin;
use std::{
    str,
    time,
    thread,
};

const QUEUE_URL: &str = "amqp://rust_rabbitmq_example_queue//";
const QUEUE_NAME: &str = "example-queue";

/// Generates a session and a channel for a consumer or producer.
/// Terminates the program if either the session, channel or queue can be created.
///
/// Returns:
///
/// (Session, Channel, Result)
fn create_session_and_channel() -> (Session, Channel, DeclareOk) {

    let mut session = Session::open_url(QUEUE_URL).unwrap();
    let mut channel = session.open_channel(1).unwrap();

    /* TODO: add parameters documentation */
    let queue = channel.queue_declare(
        QUEUE_NAME,
        false,
        true,
        false,
        false,
        false,
        Table::new()
    ).unwrap();

    return (session, channel, queue);
}

/// Correctly terminates the given session and channel, sterminate a successfull reply code with close-ok message.
/// Terminates the program immediately if the channel cannot be closed.
///
/// Args:
///
/// `session` - the session to close
/// `channel` - the channel to close
fn terminate_session_and_channel(
    mut session: Session,
    mut channel: Channel,
) {

    const CLOSE_REPLY_CODE: u16 = 200;
    const CLOSE_REPLY_TEXT: &str = "closing producer";
    channel.close(
        CLOSE_REPLY_CODE,
        CLOSE_REPLY_TEXT,
    ).unwrap();
    session.close(
        CLOSE_REPLY_CODE,
        CLOSE_REPLY_TEXT,
    );
}

/// Simulates a consumer (worker). Continuously checks for messages from the queue.
fn get_queue_messages() {

    /* initializes the producer */

    let mut _initializers = create_session_and_channel();

    let _session = _initializers.0;
    let mut channel = _initializers.1;
    let _queue = _initializers.2;

    /* TODO: explain parameters */

    let _consumer = channel.basic_consume(
        move |
            _chan: &mut Channel,
            _deliver: Deliver,
            _headers: BasicProperties,
            data: Vec<u8>,
        | {
            let message = str::from_utf8(&data).unwrap();
            println!("Start handling message: {}", message);

            /* simulate a working task */
            const TASK_SECONDS_DURATION: u64 = 3;
            thread::sleep(time::Duration::from_secs(TASK_SECONDS_DURATION));

            println!("End handling message: {}", message);
        },
        QUEUE_NAME,
        "",
        false,
        false,
        false,
        false,
        Table::new(),
    );

    channel.start_consuming();

    /* correctly terminate the session and channel */

    terminate_session_and_channel(
        _session,
        channel,
    );
}

fn main() {

    App::new("rust-rabbitmq-example")
        .version("0.0.1")
        .about("Simulate a RabbitMQ environment with consumer(s) and producer(s).")
        .get_matches();

    /* initializes the consumer */

    spawn(|| { get_queue_messages() });

    /* initializes the producer */

    let initializers = create_session_and_channel();

    let session = initializers.0;
    let mut channel = initializers.1;
    let _queue = initializers.2;

    /* user actions */

    loop {

        let mut input = String::new();
        stdin().read_line(&mut input).expect("cannot get user input");

        let input = input.trim();
        let splitted: Vec<&str> = input.split(' ').collect();

        let command: &str = match splitted.get(0) {
            Some(value) => value,
            None => { continue }
        };

        if command == "exit" {
            break;
        }
        else if command == "push" {

            let message = match splitted.get(1) {
                Some(value) => value,
                None => {
                    println!("Missing message.");
                    continue;
                }
            };

            /* TODO: add parameters documentation */

            channel.basic_publish(
                "",
                QUEUE_NAME,
                true,
                false,
                BasicProperties {
                    content_type: Some("text".to_string()),
                    ..Default::default()
                },
                message.to_string().into_bytes(),
            ).unwrap();
        }
    }

    /* send a successfull reply-code with a close-ok message
       as we simply close the connection without any error
       from the producer side */

    terminate_session_and_channel(
        session,
        channel,
    );
}
