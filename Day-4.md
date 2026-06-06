## Day 4

Today I fully converted MemeLang to a meme-style syntax.

I also optimized the interpreter for release builds using:

```toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
panic = "abort"
strip = true
```

These optimizations make MemeLang faster, smaller, and more efficient when compiled in release mode.

The language now includes meme-inspired keywords for variables, printing, conditions, loops, functions, and returns.

There are still some bugs to fix and many features to improve, but MemeLang is continuing to grow and become more usable with each update.
