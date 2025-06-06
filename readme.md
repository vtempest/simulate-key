
# Simulate Key

A simple Rust library for simulating keyboard input using the `enigo` crate. This library provides an easy-to-use interface for sending key combinations programmatically.

## Features

- **Simple API**: Simulate key combinations with string-based input
- **Comprehensive Key Support**: Function keys (F1-F24), navigation, numpad, media keys, and more
- **Modifier Support**: Ctrl, Shift, Alt, Meta/Win/Cmd combinations
- **Key Hold**: Hold keys for specified durations
- **Cross-platform**: Works on Windows, macOS, and Linux via `enigo`

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
key-simulator = "0.1.0"
```

## Usage

```rust
use simulate_key::simulate_key;

fn main() {
    // Basic key combinations
    simulate_key("ctrl+c").unwrap();
    simulate_key("alt+tab").unwrap();
    simulate_key("ctrl+shift+t").unwrap();
    
    // Function keys
    simulate_key("f5").unwrap();
    simulate_key("ctrl+f12").unwrap();
    
    // Navigation
    simulate_key("ctrl+home").unwrap();
    simulate_key("shift+end").unwrap();
    
    // Single characters
    simulate_key("a").unwrap();
    simulate_key("enter").unwrap();
    
    // Hold keys
    use simulate_key::simulate_key_hold;
    simulate_key_hold("space", 500).unwrap(); // Hold space for 500ms
}
```

## Supported Keys

### Modifiers
- `ctrl`, `control`
- `shift`
- `alt`
- `meta`, `win`, `cmd`, `command`

### Function Keys
- `f1` through `f24`

### Navigation
- `home`, `end`
- `pageup`, `pgup`, `pagedown`, `pgdn`
- `left`, `right`, `up`, `down`
- `insert`, `ins`, `delete`, `del`

### Special Keys
- `enter`, `return`
- `tab`, `space`, `backspace`
- `escape`, `esc`
- `capslock`, `numlock`, `scrolllock`
- `printscreen`, `prtsc`, `pause`

### Numpad
- `numpad0` through `numpad9`
- `numpadenter`, `numpadplus`, `numpadminus`
- `numpadmultiply`, `numpaddivide`, `numpaddot`

### Media Keys
- `volumeup`, `volumedown`, `volumemute`
- `mediaplay`, `mediastop`, `medianext`, `mediaprev`

### Single Characters
Any single character (letters, numbers, symbols)

## Error Handling

The library returns `ParseKeyError` for invalid key combinations:

```rust
match simulate_key("invalid+key") {
    Ok(()) => println!("Key simulated successfully"),
    Err(e) => println!("Error: {}", e),
}
```

