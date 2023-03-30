# Parallel Frequency Analysis
This library provides a function `frequency()` which counts the occurrence of each alphabetic case-insensitive Unicode character in the input string. It is my solution to the "Parallel Letter Frequency" exercise on Exercism.org.

# Usage
Use `cargo test` to run tests.

# Concepts Reviewed
- Rayon (`ThreadPoolBuilder`, `.par_iter()`) 
- UnicodeSegmentation (`.graphemes()`)
- `.is_alphabetic()`
