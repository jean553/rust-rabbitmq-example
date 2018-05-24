extern crate amqp;

use amqp::Session;

use std::thread::spawn;

/// Simulates a consumer (worker). Continuously checks for messages from the queue.
fn get_queue_messages() {
}

fn main() {

    const QUEUE_URL: &str = "amqp://rust_rabbitmq_example_queue//";
    let mut session = Session::open_url(QUEUE_URL).unwrap();
    let mut channel = session.open_channel(1).unwrap();

    spawn(|| { get_queue_messages() });

    const CLOSE_REPLY_CODE: u16 = 200;
    const CLOSE_REPLY_TEXT: &str = "closing";
    channel.close(
        CLOSE_REPLY_CODE,
        CLOSE_REPLY_TEXT,
    ).unwrap();
    session.close(
        CLOSE_REPLY_CODE,
        CLOSE_REPLY_TEXT,
    );
}
