@echo off
setlocal enabledelayedexpansion
set ROOT_DIR=%~dp0
set GUI_DIR=%ROOT_DIR%src\web_GUI
set OUTPUT_DIR=%ROOT_DIR%build
set APP_NAME=openhub
set BIN_NAME=%APP_NAME%.exe

echo === OpenHub Windows Builder ===
echo.

:: ── Check/Install Rust ──────────────────────────────────
echo === Checking Rust ===
where rustc >nul 2>&1
if %errorlevel% neq 0 (
    echo Rust not found. Installing via rustup...
    where winget >nul 2>&1
    if !errorlevel! equ 0 (
        winget install Rustlang.Rustup
    ) else (
        echo Downloading rustup-init.exe...
        curl -sL https://win.rustup.rs -o "%TEMP%\rustup-init.exe"
        "%TEMP%\rustup-init.exe" -y
        del "%TEMP%\rustup-init.exe"
    )
    echo.
    echo Rust installed. Restart the terminal and re-run build.bat.
    pause
    exit /b 1
)
echo Rust: OK
rustc --version

:: Add cargo to PATH for current session
where cargo >nul 2>&1 || set PATH=%USERPROFILE%\.cargo\bin;%PATH%

:: ── Check/Install Node.js ───────────────────────────────
echo.
echo === Checking Node.js ===
where node >nul 2>&1
if %errorlevel% neq 0 (
    echo Node.js not found. Installing...
    where winget >nul 2>&1
    if !errorlevel! equ 0 (
        winget install OpenJS.NodeJS
    ) else (
        echo Please install Node.js from https://nodejs.org and re-run.
        pause
        exit /b 1
    )
    echo.
    echo Node.js installed. Restart the terminal and re-run build.bat.
    pause
    exit /b 1
)
echo Node.js: OK
node --version

:: ── Check/Install pnpm ──────────────────────────────────
echo.
echo === Checking pnpm ===
where pnpm >nul 2>&1
if %errorlevel% neq 0 (
    echo Installing pnpm via npm...
    call npm install -g pnpm
)
echo pnpm: OK
pnpm --version

:: ── Update WebView2 (included in Win10+ and Edge) ───────
echo.
echo === Checking WebView2 ===
:: WebView2 is built into Windows 10 (version 1803+) and Edge.
:: If missing, the app will show a prompt to download it automatically.
echo WebView2: OK (built into Windows 10+)

:: ── Build frontend ──────────────────────────────────────
echo.
echo === Building frontend ===
cd /d "%GUI_DIR%"
call pnpm install
if %errorlevel% neq 0 (
    echo pnpm install failed
    pause
    exit /b 1
)
call pnpm build
if %errorlevel% neq 0 (
    echo Frontend build failed
    pause
    exit /b 1
)
echo Frontend: OK

:: ── Build Rust backend ──────────────────────────────────
echo.
echo === Building Rust backend (Windows) ===
cd /d "%ROOT_DIR%"
cargo build --release
if %errorlevel% neq 0 (
    echo Rust build failed
    pause
    exit /b 1
)
echo Backend: OK

:: ── Deploy to output directory ──────────────────────────
echo.
echo === Deploying to %OUTPUT_DIR% ===
if exist "%OUTPUT_DIR%" rmdir /s /q "%OUTPUT_DIR%"
mkdir "%OUTPUT_DIR%\services"
mkdir "%OUTPUT_DIR%\web"

set BIN_SRC=%ROOT_DIR%target\release\%BIN_NAME%
if exist "%BIN_SRC%" (
    copy "%BIN_SRC%" "%OUTPUT_DIR%\%BIN_NAME%"
    echo Binary copied
) else (
    echo WARNING: Binary not found at %BIN_SRC%
)

if exist "%GUI_DIR%\dist" (
    xcopy /e /i /q "%GUI_DIR%\dist\*" "%OUTPUT_DIR%\web\"
    echo Web files copied
)

if exist "%ROOT_DIR%data" (
    mkdir "%OUTPUT_DIR%\data"
    xcopy /e /i /q "%ROOT_DIR%data\*" "%OUTPUT_DIR%\data\"
    echo Data files copied
)

echo.
echo === Build Complete ===
echo Output: %OUTPUT_DIR%
echo.
echo To run:
echo   cd /d "%OUTPUT_DIR%" && %BIN_NAME%
echo.

endlocal
