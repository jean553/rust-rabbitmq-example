extern crate amqp;

use amqp::Session;

fn main() {

    const QUEUE_URL: &str = "amqp://rust_rabbitmq_example_queue//";
    let mut session = Session::open_url(QUEUE_URL).unwrap();
}
