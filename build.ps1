# usage:
#   .\build.ps1            dpad version plus 0f/1f/2f/3f
#   .\build.ps1 2f         only the 2f locked version
#   .\build.ps1 0f 2f      only the listed versions
$ErrorActionPreference = "Stop"
Set-Location -LiteralPath $PSScriptRoot

$src = "target/aarch64-skyline-switch/release/libssbu_combat_latency_slider.nro"
$out = "dist"
New-Item -ItemType Directory -Force -Path $out | Out-Null

function Build([string]$feature, [string]$suffix) {
    if ($feature) {
        cargo skyline build --release --features $feature
    } else {
        cargo skyline build --release
    }
    if ($LASTEXITCODE -ne 0) { throw "build failed (feature: $feature)" }
    Copy-Item -LiteralPath $src -Destination "$out/libssbu_combat_latency_slider$suffix.nro" -Force
    Write-Host "  -> $out/libssbu_combat_latency_slider$suffix.nro"
}

if ($args.Count -gt 0) {
    foreach ($f in $args) { Build $f "_$f" }
} else {
    Build "" ""
    Build "0f" "_0f"
    Build "1f" "_1f"
    Build "2f" "_2f"
    Build "3f" "_3f"
}
Write-Host "done -> $out/"
