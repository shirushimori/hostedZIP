@echo off
setlocal enabledelayedexpansion

set ROOT_DIR=%~dp0
set GUI_DIR=%ROOT_DIR%src\web_GUI
set OUTPUT_DIR=%ROOT_DIR%build
set APP_NAME=openhub
set BIN_NAME=%APP_NAME%.exe
set BIN_SRC=%ROOT_DIR%target\release\%BIN_NAME%

:: Add cargo and MinGW to PATH
set PATH=%USERPROFILE%\.cargo\bin;%PATH%
if exist "C:\mingw64\mingw64\bin" set PATH=C:\mingw64\mingw64\bin;%PATH%

:: ════════════════════════════════════════════════════════════
::  HEADER
:: ════════════════════════════════════════════════════════════
cls
echo.
echo  ██████╗ ██████╗ ███████╗███╗   ██╗██╗  ██╗██╗   ██╗██████╗
echo  ██╔══██╗██╔══██╗██╔════╝████╗  ██║██║  ██║██║   ██║██╔══██╗
echo  ██║  ██║██████╔╝█████╗  ██╔██╗ ██║███████║██║   ██║██████╔╝
echo  ██║  ██║██╔═══╝ ██╔══╝  ██║╚██╗██║██╔══██║██║   ██║██╔══██╗
echo  ██████╔╝██║     ███████╗██║ ╚████║██║  ██║╚██████╔╝██████╔╝
echo  ╚═════╝ ╚═╝     ╚══════╝╚═╝  ╚═══╝╚═╝  ╚═╝ ╚═════╝ ╚═════╝
echo.
echo  Windows Builder  ^|  v0.1.0
echo  ════════════════════════════════════════════════════════════
echo.

:: ════════════════════════════════════════════════════════════
::  BUILD MODE SELECTION
:: ════════════════════════════════════════════════════════════
echo  Choose build mode:
echo.
echo    [1]  Rebuild       ^(incremental — reuse cached Rust + node_modules^)
echo    [2]  Clean Build   ^(wipe build/ target/ dist/ and start fresh^)
echo    [3]  Exit
echo.
set /p BUILD_CHOICE= Enter choice [1/2/3]: 

if "%BUILD_CHOICE%"=="1" goto :MODE_REBUILD
if "%BUILD_CHOICE%"=="2" goto :MODE_CLEAN
if "%BUILD_CHOICE%"=="3" goto :EXIT_NOW
echo  Invalid choice. Please enter 1, 2, or 3.
pause
goto :EOF

:: ════════════════════════════════════════════════════════════
:MODE_REBUILD
:: ════════════════════════════════════════════════════════════
set BUILD_MODE=Rebuild
set CLEAN_CARGO=0
set CLEAN_DIST=0
set CLEAN_OUT=1
echo.
echo  [MODE] Rebuild ^(incremental^)
echo  ────────────────────────────────────────────────────────────
goto :CHECK_DEPS

:: ════════════════════════════════════════════════════════════
:MODE_CLEAN
:: ════════════════════════════════════════════════════════════
set BUILD_MODE=Clean Build
set CLEAN_CARGO=1
set CLEAN_DIST=1
set CLEAN_OUT=1
echo.
echo  [MODE] Clean Build ^(full wipe^)
echo  ────────────────────────────────────────────────────────────

echo.
echo  [CLEAN] Removing Rust target cache...
if exist "%ROOT_DIR%target" (
    cargo clean >nul 2>&1
    echo         target/  cleared
) else (
    echo         target/  not found, skipping
)

echo  [CLEAN] Removing frontend dist...
if exist "%GUI_DIR%\dist" (
    rmdir /s /q "%GUI_DIR%\dist"
    echo         dist/    cleared
) else (
    echo         dist/    not found, skipping
)

echo  [CLEAN] Removing node_modules...
if exist "%GUI_DIR%\node_modules" (
    rmdir /s /q "%GUI_DIR%\node_modules"
    echo         node_modules/ cleared
) else (
    echo         node_modules/ not found, skipping
)

echo  [CLEAN] Removing previous build output...
if exist "%OUTPUT_DIR%" (
    rmdir /s /q "%OUTPUT_DIR%"
    echo         build/   cleared
) else (
    echo         build/   not found, skipping
)

echo.
echo  Clean complete. Starting fresh build...
echo  ────────────────────────────────────────────────────────────
goto :CHECK_DEPS

:: ════════════════════════════════════════════════════════════
:CHECK_DEPS
:: ════════════════════════════════════════════════════════════
echo.
echo  [DEPS] Checking dependencies...
echo.

:: ── Rust ────────────────────────────────────────────────────
where rustc >nul 2>&1
if %errorlevel% neq 0 (
    echo  [DEPS] Rust not found. Installing via rustup...
    where winget >nul 2>&1
    if !errorlevel! equ 0 (
        winget install Rustlang.Rustup
    ) else (
        echo  Downloading rustup-init.exe...
        curl.exe -sL https://win.rustup.rs -o "%TEMP%\rustup-init.exe"
        "%TEMP%\rustup-init.exe" -y
        del "%TEMP%\rustup-init.exe"
    )
    echo.
    echo  Rust installed. Restart the terminal and re-run build.bat.
    pause
    exit /b 1
)
for /f "tokens=*" %%v in ('rustc --version 2^>nul') do set RUSTC_VER=%%v
echo         Rust    OK  ^(%RUSTC_VER%^)

:: ── Rust GNU target ─────────────────────────────────────────
rustup target list --installed 2>nul | find "x86_64-pc-windows-gnu" >nul 2>&1
if %errorlevel% neq 0 (
    echo         Installing x86_64-pc-windows-gnu target...
    rustup target add x86_64-pc-windows-gnu >nul
    rustup default stable-x86_64-pc-windows-gnu >nul
)
echo         GNU target OK

:: ── MinGW-w64 ───────────────────────────────────────────────
if not exist "C:\mingw64\mingw64\bin\gcc.exe" (
    echo         MinGW-w64 not found. Downloading...
    powershell -Command "Invoke-WebRequest -Uri 'https://github.com/niXman/mingw-builds-binaries/releases/download/14.2.0-rt_v12-rev0/x86_64-14.2.0-release-win32-seh-msvcrt-rt_v12-rev0.7z' -OutFile '%TEMP%\mingw64.7z' -UseBasicParsing"
    if not exist "C:\Program Files\7-Zip\7z.exe" (
        echo  7-Zip not found. Please install MinGW-w64 manually or install 7-Zip.
        pause
        exit /b 1
    )
    "C:\Program Files\7-Zip\7z.exe" x "%TEMP%\mingw64.7z" -o"C:\mingw64" -y >nul
    set PATH=C:\mingw64\mingw64\bin;%PATH%
    echo         MinGW-w64 installed
) else (
    echo         MinGW-w64 OK
)

:: ── Node.js ─────────────────────────────────────────────────
where node >nul 2>&1
if %errorlevel% neq 0 (
    echo         Node.js not found. Installing...
    where winget >nul 2>&1
    if !errorlevel! equ 0 (
        winget install OpenJS.NodeJS
    ) else (
        echo  Please install Node.js from https://nodejs.org and re-run.
        pause
        exit /b 1
    )
    echo.
    echo  Node.js installed. Restart the terminal and re-run build.bat.
    pause
    exit /b 1
)
for /f "tokens=*" %%v in ('node --version 2^>nul') do set NODE_VER=%%v
echo         Node.js OK  ^(%NODE_VER%^)

:: ── pnpm ────────────────────────────────────────────────────
where pnpm.cmd >nul 2>&1
if %errorlevel% neq 0 (
    echo         Installing pnpm via npm...
    call npm install -g pnpm >nul 2>&1
)
for /f "tokens=*" %%v in ('pnpm.cmd --version 2^>nul') do set PNPM_VER=%%v
echo         pnpm    OK  ^(%PNPM_VER%^)

:: ── WebView2 ────────────────────────────────────────────────
reg query "HKLM\SOFTWARE\WOW6432Node\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00FB3A3A6E2E}" /v pv >nul 2>&1
if %errorlevel% neq 0 (
    echo         WebView2 not found. Installing silently...
    powershell -Command "Invoke-WebRequest -Uri 'https://go.microsoft.com/fwlink/p/?LinkId=2124703' -OutFile '%TEMP%\MicrosoftEdgeWebview2Setup.exe' -UseBasicParsing" >nul 2>&1
    "%TEMP%\MicrosoftEdgeWebview2Setup.exe" /silent /install
    echo         WebView2 installed
) else (
    echo         WebView2 OK
)

:: ════════════════════════════════════════════════════════════
:BUILD_FRONTEND
:: ════════════════════════════════════════════════════════════
echo.
echo  [1/3] Building frontend...
echo  ────────────────────────────────────────────────────────────
cd /d "%GUI_DIR%"

:: Always re-install if node_modules is missing (clean build removed it)
if not exist "node_modules" (
    echo         pnpm install...
    call pnpm.cmd install --shamefully-hoist --config.confirmModulesPurge=false
    if !errorlevel! neq 0 (
        echo.
        echo  ERROR: pnpm install failed
        pause
        exit /b 1
    )
) else (
    echo         node_modules present, skipping install
)

:: Build with tsc + vite directly (avoids pnpm no-tty issues)
echo         Compiling TypeScript...
node node_modules\typescript\bin\tsc -b
if %errorlevel% neq 0 (
    echo.
    echo  ERROR: TypeScript compilation failed
    pause
    exit /b 1
)

echo         Bundling with Vite...
node node_modules\vite\bin\vite.js build
if %errorlevel% neq 0 (
    echo.
    echo  ERROR: Vite build failed
    pause
    exit /b 1
)
echo         Frontend: OK

:: ════════════════════════════════════════════════════════════
:BUILD_RUST
:: ════════════════════════════════════════════════════════════
echo.
echo  [2/3] Building Rust backend...
echo  ────────────────────────────────────────────────────────────
cd /d "%ROOT_DIR%"

cargo build --release
if %errorlevel% neq 0 (
    echo.
    echo  ERROR: Rust build failed
    pause
    exit /b 1
)
echo         Backend: OK

:: ════════════════════════════════════════════════════════════
:DEPLOY
:: ════════════════════════════════════════════════════════════
echo.
echo  [3/3] Deploying to build\...
echo  ────────────────────────────────────────────────────────────
cd /d "%ROOT_DIR%"

:: Wipe and recreate output layout
if exist "%OUTPUT_DIR%" rmdir /s /q "%OUTPUT_DIR%"
mkdir "%OUTPUT_DIR%\services"
mkdir "%OUTPUT_DIR%\web"
mkdir "%OUTPUT_DIR%\data"

:: Copy binary
if exist "%BIN_SRC%" (
    copy /y "%BIN_SRC%" "%OUTPUT_DIR%\%BIN_NAME%" >nul
    echo         openhub.exe copied
) else (
    echo  WARNING: Binary not found at %BIN_SRC%
)

:: Copy WebView2 loader DLL if present
if exist "%ROOT_DIR%WebView2Loader.dll" (
    copy /y "%ROOT_DIR%WebView2Loader.dll" "%OUTPUT_DIR%\WebView2Loader.dll" >nul
    echo         WebView2Loader.dll copied
)

:: Copy frontend
if exist "%GUI_DIR%\dist" (
    xcopy /e /i /q "%GUI_DIR%\dist\*" "%OUTPUT_DIR%\web\" >nul
    echo         web\ assets copied
)

:: Copy data
if exist "%ROOT_DIR%data" (
    xcopy /e /i /q "%ROOT_DIR%data\*" "%OUTPUT_DIR%\data\" >nul
    echo         data\ files copied
)

:: ════════════════════════════════════════════════════════════
:DONE
:: ════════════════════════════════════════════════════════════
echo.
echo  ════════════════════════════════════════════════════════════
echo   BUILD COMPLETE  ^|  Mode: %BUILD_MODE%
echo  ════════════════════════════════════════════════════════════
echo.
echo   Output : %OUTPUT_DIR%
echo   Binary : %OUTPUT_DIR%\%BIN_NAME%
echo.
echo   To run  (normal)  :  build\openhub.exe
echo   To run  (dev mode):  build\openhub.exe --dev
echo.
echo  ════════════════════════════════════════════════════════════
echo.
pause
goto :EOF

:EXIT_NOW
echo.
echo  Build cancelled.
echo.
endlocal
