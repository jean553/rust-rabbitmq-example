extern crate amqp;

use amqp::{
    Session,
    Channel,
    Table,
    Basic,
};

use amqp::protocol::queue::DeclareOk;

use amqp::protocol::basic::BasicProperties;

use std::thread::spawn;
use std::io::stdin;

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
    let _channel = _initializers.1;
    let _queue = _initializers.2;

    /* TODO: worker of messages */

    /* correctly terminate the session and channel */

    terminate_session_and_channel(
        _session,
        _channel,
    );
}

fn main() {

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
        if input == "exit" {
            break;
        }
        else if input == "push" {

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
                "default message".to_string().into_bytes(),
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
