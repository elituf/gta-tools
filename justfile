set windows-shell := ["pwsh", "-NoLogo", "-NoProfileLoadTime", "-Command"]

install:
    cargo build --release
    cp .\target\release\gta-tools.exe ~\.cargo\bin
    cp .\target\release\gta-tools.exe ~\Documents
