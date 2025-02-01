# JSONFixer4J 🛠️

A lightweight Java library for automatically correcting broken JSON strings. Whether you have mismatched brackets, missing commas, or incomplete literals, **JSONFixer4J** tries its best to fix them so you can keep on coding without interruptions! ⚙️

## Features
- 🧩 **Bracket Correction**: Fixes unmatched `{` or `}`.
- 🎯 **Comma Insertion**: Inserts missing commas between keys/values.
- 🔤 **String Completion**: Completes unterminated strings.
- ⚡ **Literal Recovery**: Fixes partial boolean (`true`, `false`) or null (`null`) literals.

## Quick Start

### Installation

Add **JSONFixer4J** to your Gradle project:

```groovy
repositories {
    mavenCentral()
    maven { url "https://jitpack.io" }
}

dependencies {
    // ...
    implementation 'com.github.DedInc:jsonfixer4j:b7a6d1109c'
}
```

> **Note**: Above is an example of a **build.gradle** file. If JSONFixer4J is not yet available via JitPack, you might need to install it locally or reference it as a local library.

### Usage

In your Java code, simply create an instance of `JSONAutoCorrector` and call `autocorrect(...)` with your broken JSON string.

```java
package org.example;

import com.github.dedinc.jsonfixer4j.JSONAutoCorrector;

public class Main {
    public static void main(String[] args) {
        final JSONAutoCorrector corrector = new JSONAutoCorrector();

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

This will attempt to fix each broken JSON string and print both the original and fixed versions in the console.

---

Made with ❤️ and Java.
