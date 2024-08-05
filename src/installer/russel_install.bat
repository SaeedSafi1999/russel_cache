@echo off
:: Check for administrative privileges
net session >nul 2>&1
if %errorLevel% == 0 (
    echo Running with administrative privileges.
) else (
    echo Requesting administrative privileges.
    powershell -Command "Start-Process cmd -ArgumentList '/c %~fnx0' -Verb RunAs"
    exit
)
:: Get the current path
set CURRENT_PATH=%~dp0

:: Run your Rust executable from the current path
"%CURRENT_PATH%russel.exe" -- install_service 
