#!/usr/bin/env pwsh
$ErrorActionPreference = 'Stop'

$Root = Split-Path -Parent $PSScriptRoot
$Www = Join-Path $Root 'www'
$Target = 'wasm32-unknown-unknown'
$Profile = 'wasm-release'
$CrateName = 'simple-2d-bevy-game'
$OutName = 'simple_2d_game'
$WasmPath = Join-Path $Root "target/$Target/$Profile/$CrateName.wasm"
$WasmOut = Join-Path $Www "${OutName}_bg.wasm"

Push-Location $Root
try {
    if (-not (rustup target list --installed | Select-String -SimpleMatch $Target)) {
        Write-Host "Adding rust target $Target"
        rustup target add $Target
        if ($LASTEXITCODE -ne 0) { throw "rustup target add $Target failed" }
    }

    Write-Host 'Building for wasm...'
    cargo build --profile $Profile --target $Target
    if ($LASTEXITCODE -ne 0) { throw 'cargo build failed' }

    $lock = Get-Content (Join-Path $Root 'Cargo.lock') -Raw
    $requiredWasmBindgen = [regex]::Match($lock, 'name = "wasm-bindgen"\r?\nversion = "([^"]+)"').Groups[1].Value
    if (-not $requiredWasmBindgen) { throw 'Could not find wasm-bindgen version in Cargo.lock' }

    $installedWasmBindgen = $null
    try { $installedWasmBindgen = (wasm-bindgen --version) -replace '^wasm-bindgen ', '' } catch {}
    if ($installedWasmBindgen -ne $requiredWasmBindgen) {
        Write-Host "Installing wasm-bindgen-cli $requiredWasmBindgen (found: $installedWasmBindgen)"
        cargo install -f wasm-bindgen-cli --version $requiredWasmBindgen
        if ($LASTEXITCODE -ne 0) { throw 'cargo install wasm-bindgen-cli failed' }
    }

    Write-Host 'Running wasm-bindgen...'
    wasm-bindgen --out-name $OutName --out-dir $Www --target web $WasmPath
    if ($LASTEXITCODE -ne 0) { throw 'wasm-bindgen failed' }

    if (-not (Get-Command wasm-opt -ErrorAction SilentlyContinue)) {
        throw "wasm-opt not found. Install Binaryen: 'cargo install wasm-opt', 'npm install -g binaryen' or download from https://github.com/WebAssembly/binaryen/releases"
    }

    Write-Host 'Optimizing wasm with wasm-opt...'
    wasm-opt -Oz -o $WasmOut $WasmOut
    if ($LASTEXITCODE -ne 0) { throw 'wasm-opt failed' }

    Write-Host 'Done. Serve with: python -m http.server --directory www'
} finally {
    Pop-Location
}
