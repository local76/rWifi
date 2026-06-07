#!/bin/sh
# Resolve script directory and change to it
cd "$(dirname "$0")"

echo "Building AppImage package..."

# Create AppDir structure
mkdir -p AppDir/usr/bin
mkdir -p AppDir/usr/share/applications
mkdir -p AppDir/usr/share/icons/hicolor/256x256/apps

# Copy binary
if [ -f "../../dist/binaries/rwifi" ]; then
    cp ../../dist/binaries/rwifi AppDir/usr/bin/rwifi
elif [ -f "../../target/x86_64-unknown-linux-musl/release/rwifi" ]; then
    cp ../../target/x86_64-unknown-linux-musl/release/rwifi AppDir/usr/bin/rwifi
elif [ -f "../../target/release/rwifi" ]; then
    cp ../../target/release/rwifi AppDir/usr/bin/rwifi
else
    echo "Error: compiled rwifi binary not found."
    exit 1
fi

chmod 755 AppDir/usr/bin/rwifi

# Copy desktop file and icon
cp ../desktop/rwifi.desktop AppDir/usr/share/applications/org.local76.rwifi.desktop
cp ../../assets/brand/app_icon.png AppDir/usr/share/icons/hicolor/256x256/apps/org.local76.rwifi.png
cp ../../assets/brand/app_icon.png AppDir/rwifi.png

# Run appimage-builder
appimage-builder --recipe appimage-builder.yml
