#!/bin/bash
# Update PKGBUILD checksum for ascfix AUR package
# Run this AFTER publishing to crates.io

VERSION="0.6.0"

echo "Downloading ascfix v$VERSION from crates.io..."
wget -q "https://crates.io/api/v1/crates/ascfix/$VERSION/download" -O "ascfix-$VERSION.tar.gz"

if [ $? -ne 0 ]; then
    echo "❌ Error: Could not download package from crates.io"
    echo "Make sure you've published to crates.io first with: cargo publish"
    exit 1
fi

echo "Calculating checksum..."
CHECKSUM=$(sha256sum "ascfix-$VERSION.tar.gz" | awk '{print $1}')

echo ""
echo "✓ Checksum: $CHECKSUM"
echo ""

# Update PKGBUILD
sed -i "s/sha256sums=('SKIP')/sha256sums=('$CHECKSUM')/" PKGBUILD

echo "✓ PKGBUILD updated with checksum"
echo ""

# Regenerate .SRCINFO
echo "Regenerating .SRCINFO..."
makepkg --printsrcinfo > .SRCINFO

echo "✓ .SRCINFO updated"
echo ""

# Cleanup
rm "ascfix-$VERSION.tar.gz"

echo "✅ Ready to publish to AUR!"
echo ""
echo "Next steps:"
echo "  1. Review changes: git diff"
echo "  2. Test build: makepkg -si"
echo "  3. Commit: git add PKGBUILD .SRCINFO && git commit -m 'Update to v$VERSION'"
echo "  4. Push to AUR: git push"
