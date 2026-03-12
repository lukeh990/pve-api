# PVE-API

A rust API library for Proxmox Virtual Environment.

If you weren't already aware, PVE exposes an API to interact with PVE outside
of the WebUI (But I think there is a good chance the WebUI uses this same
API).

## Helpful Links

- [Proxmox Wiki Article](https://pve.proxmox.com/wiki/Proxmox_VE_API)
- [Proxmox Offical Latest API Viewer](https://pve.proxmox.com/pve-docs/api-viewer/index.html)
  You can also get the same interface as the above API viewer for whatever
  version you are running by going to: `https://pve.fqdn:8006/pve-docs/api-viewer/index.html`

## Example

```rust
use pve_api::{Pve, SimpleApi};

#[tokio::main]
async fn main() {
    let pve = Pve::builder()
        .base_url("https://pve.fqdn:8006") // Replace with PVE address
        .api_token("<TOKEN_ID>", "<SECRET>") // Create API token and input here.
        .weak_tls(true) // Allows self-signed certs
        .build()
        .unwrap();

    if let Err(e) = pve.test().await {
        eprintln!("Error Testing Connection: {}", e);
        return;
    } else {
        println!("Sucessfully Tested Connection");
    }

    let value = pve // Important: the path below must not have a leading slash.
        .simple_post("nodes/pvetest/lxc") // This creates a new LXC
        .add_parameter("node", "pvetest")
        .add_parameter(
            "ostemplate",
            "local:vztmpl/debian-13-standard_13.1-2_amd64.tar.zst",
        )
        .add_parameter("vmid", "101")
        .add_parameter("storage", "local-lvm")
        .add_parameter("net0", "name=eth0,bridge=vmbr0,ip=dhcp")
        .add_parameter("start", "1")
        .add_parameter("password", "test1234")
        .add_parameter("ostype", "debian")
        .add_parameter("features", "nesting=1")
        .add_parameter("hostname", "test-debian")
        .send()
        .await;

    match value {
        Ok(v) => println!("OK!\n{}", v),
        Err(e) => eprintln!("ERR!\n{}", e),
    }
}
```

For more info examine the docs.rs

## Project State

Right now I wouldn't consider the project _good_. It's not well tested and
just blindly accepts any parameters and doesn't provide any types for return
values.

The reason for that is that is that the PVE API is very unwieldy and this
project is just meant to be slapped together so I didn't have time to go
through each type and endpoint. Also, the provided schema by PVE seems to get
deviated from occasionally.

The reason why I made a trait called `SimpleApi` is mainly so I can make
different APIs like an enhanced or advanced API and allow the user to choose
which methods to get in IntelliSense.

The next step for this project would be to include something like an advanced
API that includes shorthand functions for common tasks like creating VMs/CTs.
Then after that's done I would set my sights on some sort of mode that would
check against the schema advertised by PVE and use that to create safer
operations.

**Q: Would I recommend using this in production?** A: Eh, I don't see why you
couldn't as it's really just a wrapper of reqwest. I'll try to keep
dependencies up-to-date. And work on more features. And even though it's
< 1.0.0 I will try to keep `SimpleApi` stable and not change it substantially.

### To Do List:

- [x] `SimpleApi` - No Checking - No Helpers
- [ ] `HelperApi` - No Checking - Helpers
  - [ ] Manage CTs helper
  - [ ] Manage VMs helper
  - [ ] Manage HA helper
  - [ ] Manage SDN helper
  - [ ] Manage firewall helper
- [ ] `EnhancedApi` - Checking Schema - Helpers(?)
  - [ ] TBD
