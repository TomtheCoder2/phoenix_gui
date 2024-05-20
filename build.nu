# Extract the version from Cargo.toml
let version = (open -r ./Cargo.toml | lines | where $it =~ "^version" | split row "=" | select 1 | str trim | str trim --char '"')

# Build the project
cargo build --release --bin phoenix_gui

# Rename the binary
let file_name = ("./target/release/phoenix_gui_v" | append $version | append ".exe" | str join "")
print $file_name
let command = ("mv -f ./target/release/phoenix_gui.exe" | append $file_name | str join " ")
print $command
mv -f ./target/release/phoenix_gui.exe $file_name