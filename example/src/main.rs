extern crate amqp;
extern crate clap;

use amqp::{
    Session,
    Channel,
    Table,
    Basic,
};

use amqp::protocol::basic::{
    BasicProperties,
    Deliver,
};

use clap::{
    App,
    Arg,
};

use std::thread::spawn;
use std::io::stdin;
use std::{
    str,
    time,
    thread,
};

const QUEUE_URL: &str = "amqp://rust_rabbitmq_example_queue_1//";
const QUEUE_NAME: &str = "example-queue";

/// Generates a session and a channel for a consumer or producer.
/// Terminates the program if either the session, channel or queue can be created.
///
/// Args:
///
/// `durable` - Indicates if the queue messages are durable (if they are written on disk in case of queue failure/stop)
/// `prefetch_count` - maximum non aknowledged messages a consumer can consume before refusing new messages
///
/// Returns:
///
/// (Session, Channel)
fn create_session_and_channel(
    durable: bool,
    prefetch_count: u16,
) -> (Session, Channel) {

    let mut session = Session::open_url(QUEUE_URL).unwrap();
    let mut channel = session.open_channel(1).unwrap();

    /* TODO: add parameters documentation */
    channel.queue_declare(
        QUEUE_NAME,
        false,
        durable,
        false,
        false,
        false,
        Table::new()
    ).unwrap();

    if prefetch_count != 0 {
        channel.basic_prefetch(prefetch_count).unwrap();
    }

    return (session, channel);
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
///
/// Args:
///
/// `consumer_index` - the index of the consumer to use for logging
/// `enable_ack` - enables acknowledgment of consumed messages
/// `durable` - indicates if the queue messages are durable (if they are written on disk in case of queue failure/stop)
/// `prefetch_count` - maximum non aknowledged messages a consumer can consume before refusing new messages
fn get_queue_messages(
    consumer_index: usize,
    enable_ack: bool,
    durable: bool,
    prefetch_count: u16,
) {

    let mut _initializers = create_session_and_channel(
        durable,
        prefetch_count,
    );
    let _session = _initializers.0;
    let mut channel = _initializers.1;

    /* TODO: explain parameters */

    let _consumer = channel.basic_consume(
        move |
            _chan: &mut Channel,
            _deliver: Deliver,
            _headers: BasicProperties,
            data: Vec<u8>,
        | {
            let message = str::from_utf8(&data).unwrap();
            println!(
                "[Consumer {}] Start handling message: {}",
                consumer_index,
                message,
            );

            /* simulate a working task */
            const TASK_SECONDS_DURATION: u64 = 3;
            thread::sleep(time::Duration::from_secs(TASK_SECONDS_DURATION));

            println!(
                "[Consumer {}] Terminate handling message: {}",
                consumer_index,
                message,
            );
        },
        QUEUE_NAME,
        "",
        false,
        enable_ack,
        false,
        false,
        Table::new(),
    );

    println!("[Consumer {}] Started.", consumer_index);

    channel.start_consuming();

    /* FIXME: this part seems never executed... */

    terminate_session_and_channel(
        _session,
        channel,
    );
}

fn main() {

    let matches = App::new("rust-rabbitmq-example")
        .version("0.0.1")
        .about("Simulate a RabbitMQ environment with consumer(s) and producer(s).")
        .arg(Arg::with_name("consumers")
             .short("c")
             .long("consumers")
             .help("Amount of consumers (default to 1)")
             .takes_value(true)
        )
        .arg(Arg::with_name("enable_ack")
             .short("e")
             .long("enable-ack")
             .help("Enable aknowledgment of consumed messages (default to false)")
             .takes_value(true)
        )
        .arg(Arg::with_name("durable")
             .short("d")
             .long("durable")
             .help("Indicates if the queue stores messages on disk in case of failure (default to false)")
             .takes_value(true)
        )
        .arg(Arg::with_name("prefetch_count")
             .short("p")
             .long("prefetch-count")
             .help("Indicates the maximum messages amount a worker can consume until it acknowledges them (basic_qos, requires acknowledgment to be enabled).")
             .takes_value(true)
        )
        .arg(Arg::with_name("fanout")
             .short("f")
             .long("fanout")
             .help("If true, an exchange is added to handle messages and forward them into two different queues.")
             .takes_value(true)
        )
        .get_matches();

    let consumers: usize = matches.value_of("consumers")
        .unwrap_or("1")
        .parse()
        .unwrap();

    let enable_ack: bool = matches.value_of("enable_ack")
        .unwrap_or("false")
        .parse()
        .unwrap();

    let durable: bool = matches.value_of("durable")
        .unwrap_or("false")
        .parse()
        .unwrap();

    let prefetch_count: u16 = matches.value_of("prefetch_count")
        .unwrap_or("0")
        .parse()
        .unwrap();

    let fanout: bool = matches.value_of("fanout")
        .unwrap_or("false")
        .parse()
        .unwrap();

    for index in 0..consumers {
        spawn(move || {
            get_queue_messages(
                index,
                enable_ack,
                durable,
                prefetch_count,
            )
        });
    }

    let initializers = create_session_and_channel(
        durable,
        prefetch_count,
    );
    let session = initializers.0;
    let mut channel = initializers.1;

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

    terminate_session_and_channel(
        session,
        channel,
    );
}
