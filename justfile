set windows-shell := ["pwsh", "-NoLogo", "-NoProfileLoadTime", "-Command"]

build:
    cargo build --release
    upx .\target\release\gta-tools.exe --best

install: build
    cp .\target\release\gta-tools.exe ~\.cargo\bin
    cp .\target\release\gta-tools.exe ~\Documents
