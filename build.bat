@echo off
echo Building SolanaShredProxy...
echo.

REM Check if Rust is installed
rustc --version >nul 2>&1
if %errorlevel% neq 0 (
    echo Rust is not installed. Please install Rust from https://rustup.rs/
    echo.
    echo After installing Rust, run this script again.
    pause
    exit /b 1
)

echo Rust is installed. Building project...
echo.

REM Build the project
cargo build --release

if %errorlevel% equ 0 (
    echo.
    echo Build successful! Binary is located at: target\release\solanashredproxy.exe
    echo.
    echo To run the application:
    echo   target\release\solanashredproxy.exe
    echo.
) else (
    echo.
    echo Build failed. Please check the error messages above.
    echo.
)

pause
