# SolanaShredProxy Build Script for PowerShell

Write-Host "Building SolanaShredProxy..." -ForegroundColor Green
Write-Host ""

# Check if Rust is installed
try {
    $rustVersion = rustc --version 2>$null
    if ($rustVersion) {
        Write-Host "Rust is installed: $rustVersion" -ForegroundColor Green
    } else {
        throw "Rust not found"
    }
} catch {
    Write-Host "Rust is not installed. Please install Rust from https://rustup.rs/" -ForegroundColor Red
    Write-Host ""
    Write-Host "After installing Rust, run this script again." -ForegroundColor Yellow
    Read-Host "Press Enter to continue"
    exit 1
}

Write-Host "Building project..." -ForegroundColor Yellow
Write-Host ""

# Build the project
try {
    cargo build --release
    if ($LASTEXITCODE -eq 0) {
        Write-Host ""
        Write-Host "Build successful! Binary is located at: target\release\solanashredproxy.exe" -ForegroundColor Green
        Write-Host ""
        Write-Host "To run the application:" -ForegroundColor Cyan
        Write-Host "  target\release\solanashredproxy.exe" -ForegroundColor White
        Write-Host ""
    } else {
        throw "Build failed with exit code $LASTEXITCODE"
    }
} catch {
    Write-Host ""
    Write-Host "Build failed. Please check the error messages above." -ForegroundColor Red
    Write-Host ""
}

Read-Host "Press Enter to continue"
