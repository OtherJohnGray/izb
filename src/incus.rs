use crate::base::*;
use std::process::Command;

fn incus(args: &[&str]) -> Command {
    let mut cmd = Command::new("incus");
    cmd.args(args);
    cmd
}

pub struct Bridge {
    name: String,
    // address: String
}

pub fn create_bridge_network(name: &str, address: &str) -> Bridge {
    perform(
        &format!("Create bridge {}", name),
        incus(&["network", "show", name]),
        incus(&["network", "create", name, "--type=bridge", &format!("ipv4.address={}", address)])
    );
    Bridge {name: name.to_owned()}
}

pub struct Instance {
    name: String
}

pub fn create_debian_vm(name: &str) -> Instance {
    perform(
        &format!("Create Debian VM {}", name),
        incus(&["config", "show", name]),
        incus(&["create", "images:debian/12", name, "--vm"])
    );

    Instance {name: name.to_owned()}
}

pub fn attach_bridge(bridge: Bridge, vm: Instance) {
    perform(
        &format!("Attach bridge {} to {}", bridge.name, vm.name),
        incus(&["config", "device", "get", &vm.name, &bridge.name, "name"]),
        incus(&["config", "device", "add", &vm.name, &bridge.name, "nic"])
    );
}