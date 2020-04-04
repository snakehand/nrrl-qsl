extern crate serde;

mod tripletex;

use std::env;
use std::ffi::OsString;

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

fn main() {
    let args: Vec<OsString> = env::args_os().collect();
    if args.len() < 2 {
        println!("usage: {} members.csv", args[0].to_string_lossy());
        return;
    }
    let ml = match tripletex::read_members(&args[1]) {
        Ok(d) => d,
        Err(e) => {
            println!("Failed: {:?}", e);
            return;
        }
    };

    for m in &ml {
        // println!("{:?}", m);
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
        // println!("Callsign: {}", callsign);
        let group = match m.category_number2.parse::<u32>() {
            Ok(g) => g,
            Err(_) => continue,
        };

        if let Some(g) = translate_group(group) {
            print!("{},{:03}\r\n", callsign, g);
        }
    }
}
