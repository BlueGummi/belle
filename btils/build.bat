@echo off
setlocal

set CC=gcc
set CFLAGS=-Wall -Wextra -pedantic
set TARGET=bfmt
set SRC=bfmt.c

:all
%CC% %CFLAGS% -o %TARGET% %SRC%
if errorlevel 1 exit /b %errorlevel%

:clean
if exist %TARGET% (
    del %TARGET%
)

exit /b
