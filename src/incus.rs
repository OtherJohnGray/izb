use crate::base::*;
use std::fs::File;
use std::io::Write;
use std::process::Command;
use std::path::Path;


fn incus(args: &[&str]) -> Command {
    let mut cmd = Command::new("incus");
    cmd.args(args);
    cmd
}

pub struct Bridge {
    name: String,
    // address: String
}

pub struct Instance {
    name: String
}

pub struct Nic {
    mac: String
}

pub fn create_profile(name: &str) {
    let filename = &format!("/opt/builder/files/{}.profile", name);
    match File::open(filename) {
        Ok(file) => {
            let mut op = incus(&["profile", "create", name]);
            op.stdin(file);
            perform(
                &format!("Create profile {}", name),
                Some(incus(&["profile", "show", name])),
                op
            );
        },
        Err(e) => {
            halt(&format!("Profile file {} could not be opened: {}", filename, e));
        }
    }
}

pub fn create_bridge(name: &str) -> Bridge {

    let file_path = format!("/etc/systemd/network/{}.netdev", name);
    let path = Path::new(&file_path);

    // Create the file, or truncate it if it already exists
    match File::create(path) {
        Ok(mut file) => {
            let content = format!(
                "[NetDev]\n\
                Name={}\n\
                Kind=bridge\n",
                name
            );
            file.write_all(content.as_bytes())
            .unwrap_or_else(|e| halt(&format!("Could not write to file {} : {}", file_path, e)));
        },
        Err(e) => {
            halt(&format!("Could not open file {} : {}", file_path, e));
        }
    }
    let mut restart = Command::new("systemctl");
    restart.args(&["restart", "systemd-networkd"]);
    perform(
        "Restart networking",
        None,
        restart
    );
    Bridge {name: name.to_owned()}
}

pub fn create_debian_vm(name: &str, profile: &str) -> Instance {
    perform(
        &format!("Create Debian VM {}", name),
        Some(incus(&["config", "show", name])),
        incus(&["create", "images:debian/12", name, "--vm", "--profile", profile])
    );
    Instance {name: name.to_owned()}
}

pub fn attach_bridge(bridge: &Bridge, vm: &Instance, mac: &str) -> Nic {
    perform(
        &format!("Attach bridge {} to {}", bridge.name, vm.name),
        Some(incus(&["config", "device", "get", &vm.name, &bridge.name, "name"])),
        incus(&["config", "device", "add", &vm.name, &bridge.name, "nic", "nictype=bridged", &format!("parent={}", &bridge.name), &format!("hwaddr={}", &mac)])
    );
    Nic {mac: mac.to_owned()}
}

pub fn start_vm(instance: &Instance){
    perform(
        &format!("Start VM {}", instance.name),
        Some(incus(&["exec", &instance.name, "ls"])),
        incus(&["start", &instance.name]),
    );
    wait(incus(&["exec", &instance.name, "ls"]), 1);
}

pub fn push_file(instance: &Instance, path: &str) {
    let source_path = format!("/opt/builder/files{}", path);
    if !Path::new(&source_path).exists() {
        halt(&format!("Source file '{}' does not exist", source_path));
    }
    perform(
        &format!("Push file {} to {}", path, instance.name),
        Some(incus(&["file", "get", &instance.name, path])),
        incus(&["file", "push", &source_path, &format!("{}:{}", instance.name, path)])
    );
}

pub fn configure_nic(nic: Nic, name: &str, ip: &str) {

}