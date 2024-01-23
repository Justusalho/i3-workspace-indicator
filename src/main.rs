use std::process::{exit, Command};

use json::JsonValue;

fn main() {

    if cfg!(target_os = "windows") {
        println!("Can't run this program on Windows, buddy");
        exit(69);
    }

    // Get i3 workspace info using i3-msg
    let workspace_info = Command::new("i3-msg")
        .arg("-t")
        .arg("get_workspaces")
        .output()
        .expect("Error: Failed to run i3-msg");

    // Convert the program output into string
    let info_as_string = &workspace_info.stdout
        .iter()
        .map(|c| *c as char)
        .collect::<String>();

    let mut parsed_info = json::parse(info_as_string).unwrap();

    let mut workspace_json_vec = parsed_info
        .members_mut()
        .map(|ws| ws.take())
        .collect::<Vec<JsonValue>>();

    for ws in &mut workspace_json_vec {
        if ws.remove("focused").as_bool().unwrap() {
            print!( r"<txt><span fgcolor='Red'> {} </span></txt>", ws.remove("name") );
        } else {
            print!( r"<txt><span fgcolor='Green'> {} </span></txt>", ws.remove("name") );
        }
    }

}
