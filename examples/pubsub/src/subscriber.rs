#![deny(warnings)]

extern crate env_logger;
#[macro_use]
extern crate rosrust;

mod msg;

fn main() {
    env_logger::init();

    // Initialize node
    rosrust::init("listener");

    // Create subscriber
    // The subscriber is stopped when the returned object is destroyed
    let _subscriber_raii = rosrust::subscribe("chatter", 2, |v: msg::std_msgs::String| {
        // Callback for handling received messages
        ros_info!("Received: {}", v.data);
    })
    .unwrap();

    // Block the thread until a shutdown signal is received
    rosrust::spin();
}
