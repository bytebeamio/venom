use librumqttd::*;
use rumqttc::*;
use serde::{Deserialize, Serialize};

use std::thread;
use std::time::Duration;

#[derive(Debug, thiserror::Error)]
enum Error {}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
struct Config {
    broker: librumqttd::Config,
}

fn main() -> Result<(), Error> {
    // start the broker under test
    thread::spawn(move || {
        let config: Config = confy::load_path("config/rumqttd.conf").unwrap();
        let mut broker = Broker::new(config.broker);
        broker.start().unwrap();
        thread::sleep(Duration::from_secs(1_000_000));
    });

    thread::sleep(Duration::from_secs(2));

    publisher_and_subscriber_qos_works_independently();
    broker_handles_qos2_publishes_and_subscribes_correctly();
    Ok(())
}

fn publisher_and_subscriber_qos_works_independently() {
    let description = "\
        Publisher and subscriber qos are independent entities in MQTT.\
        Checks if qos rules are honored\
    ";

    println!("{:?}", description);
    let mqttoptions = MqttOptions::new("test-1", "localhost", 1884);
    let (mut client, mut connection) = Client::new(mqttoptions, 10);

    let qos = QoS::AtLeastOnce;

    thread::spawn(move || {
        client.subscribe("hello/world", qos).unwrap();
        thread::sleep(Duration::from_secs(1));

        publish(&mut client, 10, 10, QoS::AtMostOnce);
        thread::sleep(Duration::from_secs(2));

        client.disconnect().unwrap();
    });

    // muliple iterators continue the state (after reconnection)
    for notification in connection.iter() {
        println!("{:?}", notification);
        match notification {
            Ok(Event::Incoming(Incoming::Publish(publish))) => assert_eq!(publish.qos, qos),
            Err(_) => break,
            _ => continue,
        }
    }
}

fn broker_handles_qos2_publishes_and_subscribes_correctly() {
    let description = "\
        Publisher and subscriber qos are independent entities in MQTT.\
        Checks if qos rules are honored\
    ";

    println!("{:?}", description);
    let mqttoptions = MqttOptions::new("test-1", "localhost", 1883);
    let (mut client, mut connection) = Client::new(mqttoptions, 10);

    let qos = QoS::ExactlyOnce;
    thread::spawn(move || {
        client.subscribe("hello/world", qos).unwrap();
        publish(&mut client, 10, 10, QoS::AtMostOnce);
        thread::sleep(Duration::from_secs(2));
        client.disconnect().unwrap();
    });

    // muliple iterators continue the state (after reconnection)
    for notification in connection.iter() {
        println!("{:?}", notification);
        match notification {
            Ok(Event::Incoming(Incoming::Publish(publish))) => assert_eq!(publish.qos, qos),
            Err(_) => break,
            _ => continue,
        }
    }
}

fn publish(client: &mut Client, count: usize, size: usize, qos: QoS) {
    for _ in 0..count {
        let payload = vec![1; size];
        client.publish("hello/world", qos, true, payload).unwrap();
    }
}
