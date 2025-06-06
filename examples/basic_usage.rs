use simulate_key::{simulate_key, simulate_key_hold, get_supported_keys};
use std::thread;
use std::time::Duration;

fn main() {
    println!("Key Simulator Demo");
    println!("==================");
    
    // Wait a moment so user can focus on target application
    println!("Starting in 3 seconds...");
    thread::sleep(Duration::from_secs(3));
    
    // Basic key simulation
    println!("Typing 'Hello World!'");
    simulate_key("h").unwrap();
    simulate_key("e").unwrap();
    simulate_key("l").unwrap();
    simulate_key("l").unwrap();
    simulate_key("o").unwrap();
    simulate_key("space").unwrap();
    simulate_key("shift+w").unwrap(); // Capital W
    simulate_key("o").unwrap();
    simulate_key("r").unwrap();
    simulate_key("l").unwrap();
    simulate_key("d").unwrap();
    simulate_key("shift+1").unwrap(); // Exclamation mark
    
    thread::sleep(Duration::from_secs(1));
    
    // Key combinations
    println!("Testing key combinations...");
    simulate_key("ctrl+a").unwrap(); // Select all
    thread::sleep(Duration::from_millis(100));
    simulate_key("ctrl+c").unwrap(); // Copy
    thread::sleep(Duration::from_millis(100));
    simulate_key("ctrl+v").unwrap(); // Paste
    
    thread::sleep(Duration::from_secs(1));
    
    // Function keys
    simulate_key("f5").unwrap(); // Refresh
    
    thread::sleep(Duration::from_secs(1));
    
    // Hold key example
    println!("Holding space for 1 second...");
    simulate_key_hold("space", 1000).unwrap();
    
    // Show supported keys
    println!("\nSupported keys:");
    let keys = get_supported_keys();
    for (i, key) in keys.iter().enumerate() {
        print!("{}", key);
        if i < keys.len() - 1 {
            print!(", ");
        }
        if (i + 1) % 10 == 0 {
            println!();
        }
    }
    println!();
}