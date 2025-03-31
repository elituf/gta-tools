set windows-shell := ["pwsh", "-NoLogo", "-NoProfileLoadTime", "-Command"]

install:
    cargo build --release
    upx .\target\release\gta-tools.exe --best
    cp .\target\release\gta-tools.exe ~\.cargo\bin\
