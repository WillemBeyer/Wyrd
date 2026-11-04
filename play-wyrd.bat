@echo off
setlocal EnableExtensions DisableDelayedExpansion
title Wyrd - play development instance
set "LAUNCHER=%HORIZON_LAUNCHER%"
set "INSTANCE=%HORIZON_INSTANCE%"
if not defined INSTANCE set "INSTANCE=wyrd-dev"
if not defined LAUNCHER if exist "%~dp0.wyrd-launcher.txt" set /p "LAUNCHER="<"%~dp0.wyrd-launcher.txt"
if not defined LAUNCHER if exist "%~dp0horizon-launcher.exe" set "LAUNCHER=%~dp0horizon-launcher.exe"
if not defined LAUNCHER for %%L in (horizon-launcher.exe) do set "LAUNCHER=%%~$PATH:L"
if not defined LAUNCHER goto missing
if not exist "%LAUNCHER%" goto missing
echo Launching instance "%INSTANCE%" from its saved build selection.
if /i "%~1"=="--check" goto check
"%LAUNCHER%" play --instance "%INSTANCE%"
set "RESULT=%ERRORLEVEL%"
if "%RESULT%"=="0" exit /b 0
echo Launch failed with exit code %RESULT%.
pause
exit /b %RESULT%
:check
echo Command: "%LAUNCHER%" play --instance "%INSTANCE%"
echo No game was launched.
exit /b 0
:missing
echo Cannot find Horizon Launcher. Set HORIZON_LAUNCHER or .wyrd-launcher.txt.
if /i "%~1"=="--check" exit /b 2
pause
exit /b 2
