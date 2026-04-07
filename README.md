# Rust-Password-Manager

A CLI password vault that stores encrypted credentials locally. Passwords are encrypted with **AES-256-GCM** before being written to disk — your plaintext password never touches the vault file.

## How it works

On first run a 256-bit key is randomly generated and saved to `secret.key` in the working directory. Every password you store is encrypted with that key using a unique random nonce, and the ciphertext is written to `output.json` alongside the nonce and metadata.

```
secret.key     — AES-256 encryption key (keep this safe, don't commit it)
output.json    — encrypted vault (useless without secret.key)
```

Losing `secret.key` means losing access to all stored passwords — there is no recovery path.

## Features

| Option | Description |
|--------|-------------|
| 1. New Password | Encrypt and store a new credential |
| 2. Search Password | Look up and decrypt a credential by name |
| 3. Remove Password | Delete a credential from the vault |
| 4. Update Password | Re-encrypt a credential with a new password |
| 5. Quit | Exit the program |

## Run

```bash
cargo run
```

## Security notes

- Encryption: AES-256-GCM (authenticated encryption — detects tampering)
- Each entry uses a fresh random 96-bit nonce so identical passwords produce different ciphertext
- `secret.key` and `output.json` are local files — add them to `.gitignore` to avoid committing sensitive data
