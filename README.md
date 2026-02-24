# 🛡️ Safe CLI Password Scrubber (Rust)

A minimal, security-focused CLI utility written in Rust that demonstrates secure secret handling, in-place memory mutation, controlled `unsafe` usage, and automatic memory zeroization.

This project is part of a collection of mini security tools designed to help Security Engineers understand **how Rust enforces memory safety — and where additional care is required when handling secrets.**

---

## 🎯 Project Objective

Build a secure CLI utility that:

- Reads a password from terminal input without echo.
- Avoids accidental heap duplication of secrets.
- Scrubs the secret in-place.
- Prevents metadata leakage.
- Automatically wipes memory when the secret goes out of scope.

This project focuses on **security reasoning**, not just Rust syntax.

---

# 🔐 Security Model

Rust guarantees:

- Memory safety
- No use-after-free
- No double free
- No data races

Rust does **not automatically guarantee**:

- Secret zeroization
- No metadata leakage
- No accidental duplication
- Safe FFI pointer handling
- Secure allocator behavior

This tool demonstrates how to design beyond compiler guarantees.
