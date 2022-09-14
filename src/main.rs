use std::thread;
use winreg::{enums::HKEY_CURRENT_USER, RegKey};

fn main() {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (roblox_key, _) = hkcu.create_subkey("SOFTWARE\\ROBLOX Corporation\\Environments\\roblox-player").unwrap();
    match roblox_key.delete_value("LaunchExp") {
        Ok(()) => println!("The Roblox desktop app is now disabled.\nRestart the patcher to enable it again."),
        Err(e) => {
            let err = e.raw_os_error().unwrap();

            if (err == 2) {
                match roblox_key.set_value("LaunchExp", &"InApp") {
                    Ok(()) => println!("The Roblox desktop app is now enabled.\nRestart the patcher to disable it again."),
                    Err(e) => println!("{}", e.kind())
                }
            } else {
                println!("{}", e.kind());
            }
        }
    };

    println!("\nThis window will automatically close in 3 seconds...");
    thread::sleep_ms(3000);
}
