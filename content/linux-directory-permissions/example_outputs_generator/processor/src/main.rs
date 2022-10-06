use std::collections::BTreeMap;

fn main() {
    let dir = std::env::args()
        .skip(1)
        .next()
        .unwrap_or("outputs".to_owned());

    let mut commands = BTreeMap::new();
    let mut outputs: BTreeMap<String, BTreeMap<String, Vec<String>>> = BTreeMap::new();

    for entry in std::fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let name = path.file_name().unwrap().to_string_lossy();
        let mut parts = name.split(":");
        let cmd_name = parts.next().unwrap();
        let mode = parts.next().unwrap();

        let contents = std::fs::read_to_string(&path).unwrap();

        if mode == "COMMAND" {
            commands.insert(cmd_name.to_owned(), contents);
        } else {
            outputs
                .entry(cmd_name.to_owned())
                .or_default()
                .entry(contents.trim().to_owned())
                .or_default()
                .push(mode.to_owned());
        }
    }

    for (cmd_name, command) in commands {
        println!(
            r#"
<div class="term">
  <div class="command"> "#
        );
        println!("    $ {}", command);
        println!("  </div>");
        for (output, modes) in outputs.get(&cmd_name).unwrap() {
            let modes_str = modes.join(" ");

            println!("  <div class='output {}'>{}</div>", modes_str, output);
        }
        println!("</div>");
    }
}
