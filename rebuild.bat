@echo off
setlocal EnableExtensions DisableDelayedExpansion
title Wyrd - rebuild development mod

rem Optional overrides: HORIZON_LAUNCHER (full exe path), HORIZON_INSTANCE (id).
set "LAUNCHER=%HORIZON_LAUNCHER%"
set "INSTANCE=%HORIZON_INSTANCE%"
if not defined INSTANCE set "INSTANCE=wyrd-dev"
set "BUNDLE=%~dp0mixins\wyrd"

if not exist "%BUNDLE%\horizon-mixin.toml" goto missing_bundle
if not defined LAUNCHER if exist "%~dp0.wyrd-launcher.txt" set /p "LAUNCHER="<"%~dp0.wyrd-launcher.txt"
if not defined LAUNCHER if exist "%~dp0horizon-launcher.exe" set "LAUNCHER=%~dp0horizon-launcher.exe"
if not defined LAUNCHER for %%L in (horizon-launcher.exe) do set "LAUNCHER=%%~$PATH:L"
if not defined LAUNCHER goto ask_launcher
goto validate_launcher

:ask_launcher
echo Horizon Launcher was not found.
echo Enter its full exe path without surrounding quotes.
set /p "LAUNCHER=Launcher path: "

:validate_launcher
if not defined LAUNCHER goto missing_launcher
if not exist "%LAUNCHER%" goto missing_launcher
echo.
echo Launcher: "%LAUNCHER%"
echo Instance: "%INSTANCE%"
echo Source:   "%BUNDLE%"
echo Profile:  performance
echo.
if /i "%~1"=="--check" goto check_ok
echo Close the Wyrd game before rebuilding.
echo Performance keeps incremental builds and avoids a startup stack overflow on this Horizon pin.
echo This builds the local mod with full native-code access to your computer.
echo Build output follows; the first build takes longer than later rebuilds.
echo.
"%LAUNCHER%" native-build "%BUNDLE%" --instance "%INSTANCE%" --profile performance --trust-native-code
set "RESULT=%ERRORLEVEL%"
if not "%RESULT%"=="0" goto build_failed
echo.
echo Build succeeded. Run play-wyrd.bat to launch the freshly built instance.
echo An already-open launcher window may still have the old version selected.
pause
exit /b 0

:check_ok
echo Paths are valid. No build was started.
echo Command: "%LAUNCHER%" native-build "%BUNDLE%" --instance "%INSTANCE%" --profile performance --trust-native-code
exit /b 0

:missing_bundle
echo ERROR: Cannot find "%BUNDLE%\horizon-mixin.toml".
echo Keep rebuild.bat in the Wyrd repository root.
set "RESULT=2"
goto failed

:missing_launcher
echo ERROR: Cannot find Horizon Launcher at "%LAUNCHER%".
echo Put its full path on the first line of .wyrd-launcher.txt next to this file,
echo or set the HORIZON_LAUNCHER environment variable.
set "RESULT=2"
goto failed

:build_failed
echo.
echo ERROR: Horizon build failed with exit code %RESULT%.
echo Read the output above; do not launch an older build expecting these edits.

:failed
if /i "%~1"=="--check" exit /b %RESULT%
pause
exit /b %RESULT%
