set windows-shell := ["pwsh", "-NoLogo", "-NoProfileLoadTime", "-Command"]

install:
    cargo build --release
    cp .\target\release\gta-tools.exe ~\.cargo\bin
    cp .\target\release\gta-tools.exe ~\Documents

lint:
    cargo clippy -- -W clippy::pedantic -W clippy::nursery -A clippy::cast_sign_loss -A clippy::cast_possible_truncation -A clippy::cast_possible_wrap

lint-full:
    cargo clippy -- -W clippy::pedantic -W clippy::nursery -W clippy::unwrap_used

lint-unwraps:
    cargo clippy -- -W clippy::unwrap_used