extern crate amqp;

use amqp::Session;

use std::thread::spawn;
use std::io::stdin;

const QUEUE_URL: &str = "amqp://rust_rabbitmq_example_queue//";

/// Simulates a consumer (worker). Continuously checks for messages from the queue.
fn get_queue_messages() {
}

fn main() {

    /* initializes the producer */

    let mut session = Session::open_url(QUEUE_URL).unwrap();
    let mut _channel = session.open_channel(1).unwrap();

    /* initializes the consumer */

    spawn(|| { get_queue_messages() });

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
