param(
    [string]$ProjectName = "",
    [switch]$Build
)

Invoke-WebRequest -Uri "https://win.rustup.rs/x86_64" -OutFile "rustup-init.exe"
Start-Process -FilePath ".\rustup-init.exe" -ArgumentList "-y" -Wait

$env:Path += ";$env:USERPROFILE\.cargo\bin"
$rustup = "$env:USERPROFILE\.cargo\bin\rustup.exe"

& $rustup update
cargo --version
rustc --version


if ($ProjectName -ne "") {
    cargo new $ProjectName
    Set-Location $ProjectName
}

if ($Build -and $ProjectName -ne "") {
    cargo build
}
