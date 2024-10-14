# Incus ZFS Builder (izb)

Incus ZFS Builder (izb) is a Rust library that provides helper functions for provisioning ZFS-on-Root VMs with zfsbootmenu using Incus.

## Features

- Create and manage Incus profiles
- Create and configure bridge networks
- Create Debian VMs with custom profiles
- Attach bridge networks to VMs
- Start VMs and push files to them
- Manage Sanoid configuration for VM snapshots

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
izb = "0.1.0"
```

Then, you can use the library in your Rust code:

```rust
use izb::{incus, sanoid};

fn main() {
    // Create a profile
    incus::create_profile("my_profile");

    // Create a bridge network
    let bridge = incus::create_bridge_network("my_bridge", "192.168.1.1/24");

    // Create a Debian VM
    let vm = incus::create_debian_vm("my_vm", "my_profile");

    // Attach the bridge to the VM
    incus::attach_bridge(&bridge, &vm);

    // Start the VM
    incus::start_vm(&vm);

    // Push a file to the VM
    incus::push_file(&vm, "/path/to/file");

    // Exclude VMs from Sanoid snapshots
    sanoid::exclude_sanoid(&["my_vm".to_string()]);
}
```

## License
This project is licensed under the Affero General Public License v3.0 (AGPL-3.0).

## Contributing
Contributions are welcome! Please feel free to submit a Pull Request.

## Support
If you have any questions or run into any problems, please open an issue in the GitHub repository.