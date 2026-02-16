# AUR Package for ascfix

This directory contains the PKGBUILD for publishing ascfix to the Arch User Repository (AUR).

## Publishing to AUR

### First-Time Setup

1. **Create AUR account** at https://aur.archlinux.org/register/

2. **Set up SSH key** for AUR:
```bash
# Generate SSH key if you don't have one
ssh-keygen -t ed25519 -C "your-email@example.com"

# Add your public key to AUR account settings
cat ~/.ssh/id_ed25519.pub
# Copy and paste to https://aur.archlinux.org/account/
```

3. **Clone the AUR repository** (first time only):
```bash
# For a new package:
git clone ssh://aur@aur.archlinux.org/ascfix.git ascfix-aur
cd ascfix-aur

# Copy PKGBUILD and .SRCINFO
cp ../aur/PKGBUILD .
cp ../aur/.SRCINFO .
```

### Updating for New Version

1. **Update checksums** in PKGBUILD:
```bash
cd aur/

# Download the crate and calculate checksum
wget https://crates.io/api/v1/crates/ascfix/0.6.0/download -O ascfix-0.6.0.tar.gz
sha256sum ascfix-0.6.0.tar.gz

# Update sha256sums in PKGBUILD with the actual hash
# Replace 'SKIP' with the actual checksum
```

2. **Test the build locally**:
```bash
makepkg -si
# This will build and install the package locally
```

3. **Generate .SRCINFO**:
```bash
makepkg --printsrcinfo > .SRCINFO
```

4. **Commit and push to AUR**:
```bash
git add PKGBUILD .SRCINFO
git commit -m "Update to v0.6.0"
git push
```

## Testing the Package

Before publishing, test the build:

```bash
# Clean build
makepkg -sc

# Install locally
makepkg -si

# Test the installed binary
ascfix --version
ascfix --help
```

## Quick Update Script

```bash
#!/bin/bash
# update-aur.sh - Quick script to update AUR package

VERSION="0.6.0"
PKGREL="1"

# Download and get checksum
wget "https://crates.io/api/v1/crates/ascfix/$VERSION/download" -O "ascfix-$VERSION.tar.gz"
CHECKSUM=$(sha256sum "ascfix-$VERSION.tar.gz" | awk '{print $1}')
rm "ascfix-$VERSION.tar.gz"

echo "Checksum: $CHECKSUM"
echo ""
echo "Update PKGBUILD with:"
echo "pkgver=$VERSION"
echo "pkgrel=$PKGREL"
echo "sha256sums=('$CHECKSUM')"
```

## Package Information

- **Package name:** ascfix
- **Description:** Automatic ASCII diagram repair tool for Markdown files
- **Upstream:** https://github.com/evoludigit/ascfix
- **License:** MIT
- **Dependencies:** None (statically linked)
- **Build dependencies:** cargo (Rust toolchain)

## Current Version

- **Version:** 0.6.0
- **Release:** 1
- **Changelog:** See [CHANGELOG.md](../CHANGELOG.md)

## Links

- **AUR Package:** https://aur.archlinux.org/packages/ascfix
- **GitHub:** https://github.com/evoludigit/ascfix
- **crates.io:** https://crates.io/crates/ascfix
