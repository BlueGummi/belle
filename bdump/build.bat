@echo off
setlocal

set CC=gcc
set CFLAGS=-Wall -Wextra -flto -O2
set SRCDIR=src
set OBJ=bdump.o
set TARGET=bdump

if "%OS%"=="Windows_NT" (
    set RM=del
    set RM_FLAGS=/Q
    set OBJ_DIR=%SRCDIR%\*.o
) else (
    set RM=rm -f
    set OBJ_DIR=%SRCDIR%\*.o
)

set RELEASE_FLAGS=-static

:all
call :build

:release
set CFLAGS=%CFLAGS% %RELEASE_FLAGS%
call :build

:build
if exist %TARGET% (
    %RM% %TARGET% %RM_FLAGS%
)
gcc %SRCDIR%\%OBJ% -o %TARGET% %CFLAGS%
if errorlevel 1 exit /b %errorlevel%

%RM% %OBJ_DIR% %RM_FLAGS%
exit /b

:run
call :all
start cmd /c .\%TARGET%

:clean
%RM% %OBJ_DIR% %RM_FLAGS%
%RM% %TARGET% %RM_FLAGS%

exit /b
