use std::env;
use std::process::Command;

use x11rb::connection::Connection;
use x11rb::protocol::randr::ConnectionExt as randr_ConnectionExt;
use x11rb::protocol::randr::*;
use x11rb::protocol::xinerama::ConnectionExt;
use x11rb::protocol::xproto::*;
use x11rb::rust_connection::RustConnection;

fn get_edid(conn: &RustConnection, output: u32) -> Option<String> {
    let edid_atom = x11rb::protocol::xproto::ConnectionExt::intern_atom(&conn, false, b"EDID")
        .ok()?
        .reply()
        .ok()?
        .atom;
    let prop = conn
        .randr_get_output_property(output, edid_atom, AtomEnum::ANY, 0, 100, false, false)
        .ok()?
        .reply()
        .ok()?;

    if prop.format == 8 && prop.num_items >= 128 {
        Some(prop.data.iter().map(|b| format!("{:02X}", b)).collect())
    } else {
        None
    }
}

fn get_sid(conn: &RustConnection, root: u32, output: u32) -> i32 {
    let monitors = conn
        .randr_get_monitors(root, true)
        .ok()
        .and_then(|r| r.reply().ok())
        .map(|r| r.monitors)
        .unwrap_or_default();

    let screens = conn
        .xinerama_query_screens()
        .ok()
        .and_then(|r| r.reply().ok())
        .map(|r| r.screen_info)
        .unwrap_or_default();

    for monitor in &monitors {
        if monitor.outputs.contains(&output) {
            for (i, screen) in screens.iter().enumerate() {
                if screen.x_org as i16 == monitor.x as i16
                    && screen.y_org as i16 == monitor.y as i16
                    && screen.width as i16 == monitor.width as i16
                    && screen.height as i16 == monitor.height as i16
                {
                    return i as i32;
                }
            }
        }
    }

    -1
}

fn emit_command(output: &str, event: &str, edid: &str, screenid: &str, args: &[String]) {
    if args.is_empty() {
        return;
    }
    let mut cmd = Command::new(&args[0]);
    cmd.args(&args[1..]);
    cmd.env("SRANDRD_OUTPUT", output)
        .env("SRANDRD_EVENT", event)
        .env("SRANDRD_EDID", edid)
        .env("SRANDRD_SCREENID", screenid);

    if let Err(e) = cmd.spawn() {
        eprintln!("Command execution failed: {}", e);
    }
}

fn process_events(conn: &RustConnection, args: &[String]) {
    let root = conn.setup().roots[0].root;
    conn.randr_select_input(root, NotifyMask::OUTPUT_CHANGE)
        .expect("Failed to select input");
    conn.flush().unwrap();

    loop {
        let event = conn.wait_for_event().unwrap();
        if let x11rb::protocol::Event::RandrNotify(event) = event {
            let OutputChange {
                timestamp: _,
                config_timestamp: _,
                window: _,
                output,
                crtc: _,
                mode: _,
                rotation: _,
                connection: state,
                subpixel_order: _,
            } = event.u.as_oc();
            let edid = get_edid(conn, output).unwrap_or_default();
            let sid = get_sid(conn, root, output);

            match state {
                x11rb::protocol::randr::Connection::CONNECTED => {
                    emit_command(
                        &output.to_string(),
                        "connected",
                        &edid,
                        &sid.to_string(),
                        args,
                    );
                }
                x11rb::protocol::randr::Connection::DISCONNECTED => {
                    emit_command(
                        &output.to_string(),
                        "disconnected",
                        &edid,
                        &sid.to_string(),
                        args,
                    );
                }
                _ => {}
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let (conn, _) = RustConnection::connect(None).expect("Failed to connect to X server");
    process_events(&conn, &args[1..]);
}
