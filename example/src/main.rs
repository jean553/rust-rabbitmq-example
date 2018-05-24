extern crate amqp;

use amqp::{Session, Channel};

use std::thread::spawn;
use std::io::stdin;

const QUEUE_URL: &str = "amqp://rust_rabbitmq_example_queue//";

/// TODO
fn create_session_and_channel() -> (Session, Channel) {

    let mut session = Session::open_url(QUEUE_URL).unwrap();
    let mut _channel = session.open_channel(1).unwrap();

    return (session, _channel);
}

/// Simulates a consumer (worker). Continuously checks for messages from the queue.
fn get_queue_messages() {

    /* initializes the producer */

    let mut _initialisers = create_session_and_channel();

    let _session = _initialisers.0;
    let _channel = _initialisers.1;

    /* worker of messages */

}

fn main() {

    /* initializes the consumer */

    spawn(|| { get_queue_messages() });

    /* initializes the producer */

    let initialisers = create_session_and_channel();

    let mut session = initialisers.0;
    let mut _channel = initialisers.1;

    /* user actions */

    let mut input = String::new();

    loop {
        stdin().read_line(&mut input).expect("cannot get user input");

        let input = input.trim();
        if input == "exit" {
            break;
        }
    }

    /* send a successfull reply-code with a close-ok message
       as we simply close the connection without any error
       from the producer side */

    const CLOSE_REPLY_CODE: u16 = 200;
    const CLOSE_REPLY_TEXT: &str = "closing producer";
    _channel.close(
        CLOSE_REPLY_CODE,
        CLOSE_REPLY_TEXT,
    ).unwrap();
    session.close(
        CLOSE_REPLY_CODE,
        CLOSE_REPLY_TEXT,
    );
}
