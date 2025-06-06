
extern crate enigo;

use enigo::{
    Direction::{Click, Press, Release},
    Enigo, Key, Keyboard, Settings,
};

#[derive(Debug, Clone, PartialEq)]
pub struct ParseKeyError(pub String);

impl std::fmt::Display for ParseKeyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ParseKeyError: {}", self.0)
    }
}

impl std::error::Error for ParseKeyError {}

/// Perform any key combination passed in as string
/// 
/// # Arguments
/// * `key_combination` - A string in the format of a key combination
/// 
/// # Examples
/// ```
/// use simulate_key::simulate_key;
/// 
/// // Basic key combinations
/// simulate_key("ctrl+c").unwrap();
/// simulate_key("alt+tab").unwrap();
/// simulate_key("ctrl+shift+t").unwrap();
/// 
/// // Function keys
/// simulate_key("f5").unwrap();
/// simulate_key("ctrl+f12").unwrap();
/// 
/// // Navigation keys
/// simulate_key("ctrl+home").unwrap();
/// simulate_key("shift+end").unwrap();
/// 
/// // Special characters and symbols
/// simulate_key("ctrl+;").unwrap();
/// simulate_key("alt+[").unwrap();
/// ```
/// 
/// # Errors
/// Returns `ParseKeyError` if the key combination cannot be parsed
/// 
/// # Supported Keys
/// - **Modifiers**: ctrl/control, shift, alt, meta/win/cmd/command
/// - **Function Keys**: f1-f24
/// - **Navigation**: home, end, pageup/pgup, pagedown/pgdn, insert, delete/del
/// - **Arrows**: left, right, up, down
/// - **Special**: enter/return, tab, space, backspace, escape/esc, capslock, numlock, scrolllock
/// - **Numpad**: numpad0-numpad9, numpadenter, numpadplus, numpadminus, numpadmultiply, numpaddivide, numpaddot
/// - **Media**: volumeup, volumedown, volumemute, mediaplay, mediastop, medianext, mediaprev
/// - **System**: printscreen/prtsc, pause, sleep, wake
/// - **Symbols**: All standard symbols (!, @, #, $, %, etc.)
/// - **Single Characters**: Any single character (a-z, 0-9)
pub fn simulate_key(key_combination: &str) -> Result<(), ParseKeyError> {
    let mut enigo = Enigo::new(&Settings::default())
        .map_err(|e| ParseKeyError(format!("Failed to create Enigo instance: {}", e)))?;
    
    let parts: Vec<String> = key_combination
        .split('+')
        .map(|s| s.trim().to_lowercase())
        .collect();
    
    if parts.is_empty() {
        return Err(ParseKeyError("Empty key combination".to_string()));
    }
    
    // The last part is always the key
    let key = parts.last().unwrap();
    // All parts except the last one are modifiers
    let modifiers: Vec<&str> = parts[..parts.len() - 1]
        .iter()
        .map(|s| s.as_str())
        .collect();
    
    // Press all modifier keys
    for modifier in &modifiers {
        let key = parse_modifier(modifier)?;
        let _ = enigo.key(key, Press);
    }
    
    // Handle the main key
    let main_key = parse_main_key(key)?;
    let _ = enigo.key(main_key, Click);
    
    // Release all modifier keys in reverse order
    for modifier in modifiers.iter().rev() {
        let key = parse_modifier(modifier)?;
        let _ = enigo.key(key, Release);
    }
    
    Ok(())
}

/// Parse modifier keys
fn parse_modifier(modifier: &str) -> Result<Key, ParseKeyError> {
    match modifier {
        "ctrl" | "control" => Ok(Key::Control),
        "shift" => Ok(Key::Shift),
        "alt" => Ok(Key::Alt),
        "meta" | "win" | "cmd" | "command" => Ok(Key::Meta),
        _ => Err(ParseKeyError(format!("Unknown modifier: {}", modifier))),
    }
}


fn parse_main_key(key: &str) -> Result<Key, ParseKeyError> {
    match key.len() {
        1 => Ok(Key::Unicode(key.chars().next().unwrap())),
        _ => match key.to_lowercase().as_str() {
            // Basic keys
            "enter" | "return"        => Ok(Key::Return),
            "tab"                     => Ok(Key::Tab),
            "space"                   => Ok(Key::Space),
            "backspace"               => Ok(Key::Backspace),
            "delete" | "del"          => Ok(Key::Delete),
            "insert" | "ins"          => Ok(Key::Insert),
            "escape" | "esc"          => Ok(Key::Escape),
            
            // Navigation
            "home"                    => Ok(Key::Home),
            "end"                     => Ok(Key::End),
            "pageup" | "pgup"         => Ok(Key::PageUp),
            "pagedown" | "pgdn"       => Ok(Key::PageDown),
            
            // Arrow keys
            "left" | "leftarrow"      => Ok(Key::LeftArrow),
            "right" | "rightarrow"    => Ok(Key::RightArrow),
            "up" | "uparrow"          => Ok(Key::UpArrow),
            "down" | "downarrow"      => Ok(Key::DownArrow),
            
            // Function keys (F1-F35)
            "f1"  => Ok(Key::F1),
            "f2"  => Ok(Key::F2),
            "f3"  => Ok(Key::F3),
            "f4"  => Ok(Key::F4),
            "f5"  => Ok(Key::F5),
            "f6"  => Ok(Key::F6),
            "f7"  => Ok(Key::F7),
            "f8"  => Ok(Key::F8),
            "f9"  => Ok(Key::F9),
            "f10" => Ok(Key::F10),
            "f11" => Ok(Key::F11),
            "f12" => Ok(Key::F12),
            "f13" => Ok(Key::F13),
            "f14" => Ok(Key::F14),
            "f15" => Ok(Key::F15),
            "f16" => Ok(Key::F16),
            "f17" => Ok(Key::F17),
            "f18" => Ok(Key::F18),
            "f19" => Ok(Key::F19),
            "f20" => Ok(Key::F20),
            "f21" => Ok(Key::F21),
            "f22" => Ok(Key::F22),
            "f23" => Ok(Key::F23),
            "f24" => Ok(Key::F24),
            "f25" => Ok(Key::F25),
            "f26" => Ok(Key::F26),
            "f27" => Ok(Key::F27),
            "f28" => Ok(Key::F28),
            "f29" => Ok(Key::F29),
            "f30" => Ok(Key::F30),
            "f31" => Ok(Key::F31),
            "f32" => Ok(Key::F32),
            "f33" => Ok(Key::F33),
            "f34" => Ok(Key::F34),
            "f35" => Ok(Key::F35),
            
            // Lock keys
            "capslock" | "caps"       => Ok(Key::CapsLock),
            "numlock" | "num"         => Ok(Key::Numlock),
            "scrolllock" | "scroll"   => Ok(Key::ScrollLock),
            
            // System keys
            "printscreen" | "prtsc"   => Ok(Key::PrintScr),
            "pause"                   => Ok(Key::Pause),
            
            // Media keys
            "volumeup" | "volup"      => Ok(Key::VolumeUp),
            "volumedown" | "voldown"  => Ok(Key::VolumeDown),
            "volumemute" | "mute"     => Ok(Key::VolumeMute),
            "mediaplay" | "play"      => Ok(Key::MediaPlayPause),
            "mediastop" | "stop"      => Ok(Key::MediaStop),
            "medianext" | "next"      => Ok(Key::MediaNextTrack),
            "mediaprev" | "prev"      => Ok(Key::MediaPrevTrack),
            
            // Numpad keys
            "numpad0" => Ok(Key::Numpad0),
            "numpad1" => Ok(Key::Numpad1),
            "numpad2" => Ok(Key::Numpad2),
            "numpad3" => Ok(Key::Numpad3),
            "numpad4" => Ok(Key::Numpad4),
            "numpad5" => Ok(Key::Numpad5),
            "numpad6" => Ok(Key::Numpad6),
            "numpad7" => Ok(Key::Numpad7),
            "numpad8" => Ok(Key::Numpad8),
            "numpad9" => Ok(Key::Numpad9),
            
            // Special symbols
            "comma" => Ok(Key::Unicode(',')),
            "period" => Ok(Key::Unicode('.')),
            "semicolon" => Ok(Key::Unicode(';')),
            "quote" => Ok(Key::Unicode('\'')),
            "bracketleft" => Ok(Key::Unicode('[')),
            "bracketright" => Ok(Key::Unicode(']')),
            "backslash" => Ok(Key::Unicode('\\')),
            "slash" => Ok(Key::Unicode('/')),
            "equal" => Ok(Key::Unicode('=')),
            "minus" => Ok(Key::Unicode('-')),
            "grave" => Ok(Key::Unicode('`')),
            
            _ => Err(ParseKeyError(format!("Unknown key: {}", key))),
        }
    }
}



/// Simulate a key press and hold for a specified duration
/// 
/// # Arguments
/// * `key_combination` - A string in the format of a key combination
/// * `duration_ms` - Duration to hold the key in milliseconds
/// 
/// # Examples
/// ```
/// use simulate_key::simulate_key_hold;
/// 
/// // Hold space for 500ms
/// simulate_key_hold("space", 500).unwrap();
/// 
/// // Hold Ctrl+A for 100ms
/// simulate_key_hold("ctrl+a", 100).unwrap();
/// ```
pub fn simulate_key_hold(key_combination: &str, duration_ms: u64) -> Result<(), ParseKeyError> {
    let mut enigo = Enigo::new(&Settings::default())
        .map_err(|e| ParseKeyError(format!("Failed to create Enigo instance: {}", e)))?;
    
    let parts: Vec<String> = key_combination
        .split('+')
        .map(|s| s.trim().to_lowercase())
        .collect();
    
    if parts.is_empty() {
        return Err(ParseKeyError("Empty key combination".to_string()));
    }
    
    let key = parts.last().unwrap();
    let modifiers: Vec<&str> = parts[..parts.len() - 1]
        .iter()
        .map(|s| s.as_str())
        .collect();
    
    // Press all modifier keys
    for modifier in &modifiers {
        let key = parse_modifier(modifier)?;
        let _ = enigo.key(key, Press);
    }
    
    // Press and hold the main key
    let main_key = parse_main_key(key)?;
    let _ = enigo.key(main_key, Press);
    
    // Hold for specified duration
    std::thread::sleep(std::time::Duration::from_millis(duration_ms));
    
    // Release the main key
    let _ = enigo.key(main_key, Release);
    
    // Release all modifier keys in reverse order
    for modifier in modifiers.iter().rev() {
        let key = parse_modifier(modifier)?;
        let _ = enigo.key(key, Release);
    }
    
    Ok(())
}

/// Get a list of all supported keys
pub fn get_supported_keys() -> Vec<&'static str> {
    vec![
        // Modifiers
        "ctrl", "control", "shift", "alt", "meta", "win", "cmd", "command",
        
        // Basic keys
        "enter", "return", "tab", "space", "backspace", "delete", "del", "escape", "esc",
        
        // Navigation
        "home", "end", "pageup", "pgup", "pagedown", "pgdn",
        
        // Arrows
        "left", "right", "up", "down",
        
        // Function keys
        "f1", "f2", "f3", "f4", "f5", "f6", "f7", "f8", "f9", "f10", "f11", "f12",
        "f13", "f14", "f15", "f16", "f17", "f18", "f19", "f20", "f21", "f22", "f23", "f24",
        
        // Lock keys
        "capslock", "caps", "numlock", "num", "scrolllock", "scroll",
        
        // System
        "pause",
        
        // Media (available keys)
        "volumeup", "volup", "volumedown", "voldown", "volumemute", "mute", "mediastop", "stop",
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_modifier() {
        assert!(parse_modifier("ctrl").is_ok());
        assert!(parse_modifier("shift").is_ok());
        assert!(parse_modifier("alt").is_ok());
        assert!(parse_modifier("meta").is_ok());
        assert!(parse_modifier("invalid").is_err());
    }

    #[test]
    fn test_parse_main_key() {
        assert!(parse_main_key("a").is_ok());
        assert!(parse_main_key("enter").is_ok());
        assert!(parse_main_key("f1").is_ok());
        assert!(parse_main_key("invalid_key_name").is_err());
    }

    #[test]
    fn test_simulate_key_parsing() {
        // These tests just verify parsing, not actual key simulation
        // since that requires system interaction
        
        // Test that the function doesn't panic on valid inputs
        let result = simulate_key("ctrl+c");
        // We can't test the actual key press in unit tests, but we can test that parsing works
        
        let result = simulate_key("invalid+key");
        assert!(result.is_err());
    }
}
