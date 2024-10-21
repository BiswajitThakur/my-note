# GnuPG

## Installation

### Debin/Ubuntu Linux
```bash
sudo apt install gpg
# or
# sudo apt install gnupg
```

### MAC
```bash
brew install gnupg
```

## Generating a new keypair
```bash
gpg --expert --full-gen-key
```

## Export Your Public Key
```bash
gpg --armor --export user-id > pubkey.asc
```

## Export Your Private Key
```bash
gpg --export-secret-keys --armor user-id > privkey.asc
```

## Delete a key
```bash
# Private keys
# Use the ID listed with --list-secret-keys
gpg --delete-secret-keys XXXXXXXX

# Public keys
# Use the ID listed with --list-keys
gpg --delete-keys XXXXXXXX
```

## Import a key
```bash
# This works the same for binary or ASCII (armored) versions of keys
# This is also the same for private and public keys
gpg --import ./my-priv-gpg-key.asc
```
<!--
## Push your public key to key server
```bash
# There are many public key servers out there, not just MIT
# Replace XXXXXXXX with your key id from --list-keys
gpg --keyserver hkp://pgp.mit.edu --send-keys XXXXXXXX
```
-->

