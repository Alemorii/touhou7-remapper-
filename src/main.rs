use evdev::{EventSummary, KeyCode};

fn main() {
    let mut device = evdev::Device::open("/dev/input/event3").unwrap();
    device.grab().unwrap();
    
    let mut output = uinput::default().unwrap()
        .name("touhou-remap").unwrap()
        .event(uinput::event::keyboard::Key::Up).unwrap()
        .event(uinput::event::keyboard::Key::Down).unwrap()
        .event(uinput::event::keyboard::Key::Left).unwrap()
        .event(uinput::event::keyboard::Key::Right).unwrap()
        .event(uinput::event::keyboard::Key::Z).unwrap()
        .event(uinput::event::keyboard::Key::X).unwrap()
        .event(uinput::event::keyboard::Key::LeftShift).unwrap() 
        .event(uinput::event::keyboard::Key::Esc).unwrap()
        .event(uinput::event::keyboard::Key::Enter).unwrap()

        .create().unwrap();

    loop {
        for event in device.fetch_events().unwrap() {
            match event.destructure() {
                EventSummary::Key(_, code, value) => {
                    println!("code : {:?} - value : {}", code, value);

                    if code == KeyCode::KEY_Q { // para cerrar el programa d1
                        std::process::exit(0);
                    }
                    
                    //la primera parte de este arrow debe ser cambiado si quieren cambiar las teclas
                    let arrow = match code {
                        KeyCode::KEY_W => Some(uinput::event::keyboard::Key::Up),
                        KeyCode::KEY_S => Some(uinput::event::keyboard::Key::Down),
                        KeyCode::KEY_A => Some(uinput::event::keyboard::Key::Left),
                        KeyCode::KEY_D => Some(uinput::event::keyboard::Key::Right),
                        KeyCode::KEY_U => Some(uinput::event::keyboard::Key::Z),
                        KeyCode::KEY_I => Some(uinput::event::keyboard::Key::X),
                        KeyCode::KEY_ESC => Some(uinput::event::keyboard::Key::Esc),
                        KeyCode::KEY_LEFTSHIFT => Some(uinput::event::keyboard::Key::LeftShift),
                        KeyCode::KEY_ENTER => Some(uinput::event::keyboard::Key::Enter),


                        _ => None,
                    };

                    if let Some(key) = arrow {
                        if value == 1 {
                            output.press(&key).unwrap();
                        } else if value == 0 {
                            output.release(&key).unwrap();
                        }
                        output.synchronize().unwrap();
                    }
                }
                _ => {}
            }
        }
    }
}
