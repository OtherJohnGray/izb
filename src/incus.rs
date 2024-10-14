use crate::base::*;
use std::path::Path;
use std::process::Command;
use std::fs::File;


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

pub fn create_profile(name: &str) {
    let filename = &format!("/opt/builder/files/{}.profile", name);
    match File::open(filename) {
        Ok(file) => {
            let mut op = incus(&["profile", "create", name]);
            op.stdin(file);
            perform(
                &format!("Create profile {}", name),
                incus(&["profile", "show", name]),
                op
            );
        },
        Err(e) => {
            halt(&format!("Profile file {} could not be opened: {}", filename, e));
        }
    }
}

pub fn create_bridge_network(name: &str, address: &str) -> Bridge {
    perform(
        &format!("Create bridge {}", name),
        incus(&["network", "show", name]),
        incus(&["network", "create", name, "--type=bridge", &format!("ipv4.address={}", address)])
    );
    Bridge {name: name.to_owned()}
}

pub fn create_debian_vm(name: &str, profile: &str) -> Instance {
    perform(
        &format!("Create Debian VM {}", name),
        incus(&["config", "show", name]),
        incus(&["create", "images:debian/12", name, "--vm", "--profile", profile])
    );
    Instance {name: name.to_owned()}
}

pub fn attach_bridge(bridge: &Bridge, vm: &Instance) {
    perform(
        &format!("Attach bridge {} to {}", bridge.name, vm.name),
        incus(&["config", "device", "get", &vm.name, &bridge.name, "name"]),
        incus(&["network", "attach", &bridge.name, &vm.name])
    );
}

pub fn start_vm(instance: &Instance){
    perform(
        &format!("Start VM {}", instance.name),
        incus(&["exec", &instance.name, "ls"]),
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
        incus(&["file", "get", &instance.name, path]),
        incus(&["file", "push", &source_path, &format!("{}:{}", instance.name, path)])
    );
}
