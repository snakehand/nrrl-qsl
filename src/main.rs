extern crate serde;

mod tripletex;

use dialog::DialogBox;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

// Translate group number code from customer file to QSL sorting code (if valid)
fn translate_group(group: u32) -> Option<u32> {
    if group == 0 {
        return None; // SK
    } else if group < 100 {
        return Some(group + 100);
    }
    let g = match group {
        175 => 19,    // Haldengruppen av NRRL
        240 => 9999,  // Østerdalsgruppen av NRRL
        318 => 24,    // Hortengruppen av NRRL
        325 => 27,    // Larvikgruppen av NRRL
        354 => 53,    // Nedre Hallingdalsgruppen av NRRL
        357 => 12,    // Hallingdalsgruppen av NRRL
        367 => 34,    // Notoddengruppen av NRRL
        375 => 29,    // Drangedalgruppen av NRRL
        400 => 9999,  // Stavangergruppen av NRRL
        413 => 42,    // Ryfylkegruppen av NRRL
        420 => 132,   // Saudagruppen
        437 => 47,    // Egersundsgruppen av NRRL
        445 => 11,    // Listagruppen av NRRL
        554 => 41,    // Haugalandgruppen av NRRL
        848 => 91,    // Andøygruppen av NRRL
        865 => 87,    // Mosjøengruppen av NRRL
        917 => 117,   // Svalbardgruppen av NRRL
        976 => 89,    // Nordkappgruppen av NRRL
        995 => 97,    // Vardøgruppen av NRRL
        9999 => 9999, // Ingen gruppetilknytning
        g => g / 10,  // Default rule
    };
    if g >= 1000 {
        None
    } else {
        Some(g)
    }
}

fn process_file(mut fname: PathBuf) -> Result<(), Box<dyn Error>> {
    let ml = tripletex::read_members(fname.as_os_str())?;

    let mut result = Vec::new();
    for m in &ml {
        if m.customer_number.is_empty() {
            continue; // Not a member
        }
        let callsign_start = match m.name.find('(') {
            Some(p) => p + 1,
            None => continue,
        };
        let callsign_end = match m.name.find(')') {
            Some(p) => p,
            None => continue,
        };
        if callsign_start > callsign_end {
            continue;
        }
        let callsign = &m.name[callsign_start..callsign_end];

        let group = match m.category_number2.parse::<u32>() {
            Ok(g) => g,
            Err(_) => continue,
        };

        if let Some(g) = translate_group(group) {
            let r = format!("{},{:03}\r\n", callsign, g);
            result.push(r);
        }
    }

    if !result.is_empty() {
        fname.set_extension("txt");
        let mut file = File::create(&fname)?;
        let all = result.join("");
        file.write_all(all.as_bytes())?;
    }

    Ok(())
}

fn main() {
    let fname = dialog::FileSelection::new("Velg CVS fil med kunde-data")
        .title("File Selection")
        .show()
        .expect("Could not display dialog box");

    let fname = match fname {
        Some(f) => f,
        None => {
            dialog::Message::new(format!("No file selected"))
                .title("Error")
                .show()
                .expect("Could not display dialog box");
            return;
        }
    };

    match process_file(PathBuf::from(fname)) {
        Ok(_) => {
            dialog::Message::new("The operation was successful.")
                .title("Success")
                .show()
                .expect("Could not display dialog box");
        }
        Err(e) => {
            dialog::Message::new(format!("Operation failed: {:?}", e))
                .title("Failure")
                .show()
                .expect("Could not display dialog box");
        }
    }
}
