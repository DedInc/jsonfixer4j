# JSONFixer4J 🛠️

A fast and efficient tool for automatically correcting broken JSON strings, written in Rust for maximum performance and with JNI for Java integration. Whether you have mismatched brackets, missing commas, or incomplete literals, **JSONFixer4J** fixes them efficiently! ⚙️

## Features
- 🧩 **Bracket Correction**: Fixes unmatched `{` or `}`.
- 🎯 **Comma Insertion**: Inserts missing commas between keys/values.
- 🔤 **String Completion**: Completes unterminated strings.
- ⚡ **Literal Recovery**: Fixes partial boolean (`true`, `false`) or null (`null`) literals.
- 🚀 **Fast**: Written inRust for exceptional performance.

## Installation

Clone the repository:
```bash
git clone https://github.com/DedInc/jsonfixer4j
```

Navigate to the Rust implementation directory:
```bash
cd jsonfixer4j/jsonfixer_rust
```

Build the project:
```bash
cargo build --release
```

The built binary will be available in the `target/release` directory with .dll or .so extension.

### Pre-built Binaries

Pre-built binaries are available in the [latest release](https://github.com/DedInc/jsonfixer4j/releases/latest) built on:
- Linux (Ubuntu 22.04.5 LTS x86_64) [.so]
- Windows 10 (x64) [.dll]

## Integration with Java via JNI

JSONFixer4J can be integrated with Java applications using JNI (Java Native Interface) via native. This allows you to leverage the performance benefits of Rust while working within your Java codebase.

### Usage Example

```java
import com.github.dedinc.jsonfixer4j.JSONFixerRust;

public class Main {
    public static void main(String[] args) {
        JSONFixerRust corrector = new JSONFixerRust();

        String[] brokenCases = {
                "{\"key\": 123",                    // Missing closing brace
                "{{\"name\": \"Test\"}",            // Extra brace at the start
                "{\"arr\": [1, 2, 3}",              // Missing closing bracket for array
                "{\"key\": \"test\", \"star, ",     // Unfinished key
                "{\"key\": \"test\", \"new\": fals",// Incomplete boolean
                "{\"title\": \"Hello",              // Unterminated string
                "{\"key1\": 1, \"key2\": 2,",       // Trailing comma
                "{\"one\": 1 \"two\": 2}",          // Missing comma
                "{\"flag\": tr, \"value\": nul}"    // Partial literals
        };

        for (String broken : brokenCases) {
            String fixed = corrector.autocorrect(broken);
            System.out.println("Broken:  " + broken);
            System.out.println("Fixed:   " + fixed);
            System.out.println("----------------------");
        }
    }
}
```

## Why Rust?

The Rust implementation provides several advantages:

1. **Performance**: Significantly faster JSON processing compared to the Java version.
2. **Memory Safety**: Rust's ownership system prevents common programming errors.
3. **No Garbage Collection**: Predictable performance without GC pauses.

---

Made with ❤️, Rust, and Java.
