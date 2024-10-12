
use crate::base::*;

const SANOID_CONF: &str = "/etc/sanoid/sanoid.conf";

pub fn exclude_sanoid(machines: &[String]) {
    match std::fs::copy(SANOID_CONF, format!("{}.bak", SANOID_CONF)) {
        Ok(_) => {
            log(&format!("Backup made to {}.bak", SANOID_CONF));
        },
        Err(e) => {
            halt(&format!("Could not make backup of {}: {}", SANOID_CONF, e))
        }
    } 

    let mut added_count = 0;
    let mut skipped_count = 0;

    for machine in machines {
        if machine_entries_exist(machine) {
            log(&format!("Entries for {} already exist in {}. Skipping.", machine, SANOID_CONF));
            skipped_count += 1;
        } else if add_machine_entries(machine) {
            added_count += 1;
        } 
    }

    if added_count > 0 {
        log(&format!("Added entries for {} machine(s). Skipped {} existing entries. Original file backed up as {}.bak", added_count, skipped_count, SANOID_CONF));
    } else {
        log(&format!("No new entries added. All {} machine(s) already had existing entries.", skipped_count));
    }

}

fn machine_entries_exist(machine: &str) -> bool {
    match std::fs::read_to_string(SANOID_CONF){
        Ok(contents) => {
            contents.contains(&format!("[rpool/lxd/virtual-machines/{}]", machine))
        },
        Err(e) => {
            halt(&format!("Could not read {} : {}", SANOID_CONF, e));
        }
    }
}

fn add_machine_entries(machine: &str) -> bool {
    match std::fs::read_to_string(SANOID_CONF) {
        Ok(mut contents) => {
            let new_entries = format!(
                "\n[rpool/lxd/virtual-machines/{}]\n\
                use_template = ignore\n\
                [rpool/lxd/virtual-machines/{}.block]\n\
                use_template = ignore\n",
                machine, machine
            );

            if let Some(pos) = contents.find("# Exclude VM ZVOLs Snapshotted by build scripts") {
                contents.insert_str(pos + "# Exclude VM ZVOLs Snapshotted by build scripts".len(), &new_entries);
                match std::fs::write(SANOID_CONF, contents) {
                    Ok(_) => {
                        log(&format!("Successfully added entries for {} to {}", machine, SANOID_CONF));
                        true
                    },
                    Err(e) => {
                        halt(&format!("Failed to insert sanoid config for {} in {}: {}", machine, SANOID_CONF, e))
                    }
                }
            } else {
                halt(&format!("Failed to find insertion point for {} in {}", machine, SANOID_CONF));
            }
        },
        Err(e) => {
            halt(&format!("Could not read file {} : {}", SANOID_CONF, e));
        }
            
    } 

}
