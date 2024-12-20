use core::time;
use std::thread;
use tokio::io::AsyncWriteExt;
use bluer::l2cap::stream::OwnedWriteHalf;

use super::hid::*;

// Sends a structured data packet to target device.
pub async fn send_keyboard_report(data: Vec<KeyboardInputs>, write_pipe: &mut OwnedWriteHalf) {
    let data_buffer = hid_raw_bytes(data);
    let written = write_pipe.write(&data_buffer).await.expect("Write failed !");
    println!("Written {} values.", written);
}

// Simulates a single keypress.
pub async fn send_keypress(data: Vec<KeyboardInputs>, write_pipe: &mut OwnedWriteHalf) {
    let dur = time::Duration::from_millis(4);

    send_keyboard_report(data, write_pipe).await;
    thread::sleep(dur);
    send_keyboard_report(vec![], write_pipe).await; // Sending empty keyboard report
    thread::sleep(dur);
}

// Sends a text through a single function call.
pub async fn send_ascii(data: String, write_pipe: &mut OwnedWriteHalf) {
    for c in data.chars() {
        let keyboard_input = ascii_to_hid(c).expect("Unable to parse character !");
        send_keypress(keyboard_input, write_pipe).await;
    }
}