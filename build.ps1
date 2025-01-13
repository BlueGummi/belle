$Clean = $false
$WithCleanup = $false

function Print-Message {
    param (
        [string]$Message,
        [string]$Color
    )

    switch ($Color) {
        "green" { Write-Host $Message -ForegroundColor Green }
        "red"   { Write-Host $Message -ForegroundColor Red }
        "yellow"{ Write-Host $Message -ForegroundColor Yellow }
        "blue"  { Write-Host $Message -ForegroundColor Blue }
        default { Write-Host $Message }
    }
}

function Clear-Line {
    Write-Host "`r`n" -NoNewline
}

function Clean {
    Print-Message "Cleaning up..." "blue"
    Set-Location basm
    cargo clean --quiet
    Set-Location ..
    Set-Location bdump
    make clean --quiet
    Set-Location ..
    Set-Location belle
    cargo clean --quiet
    Set-Location fuzz
    cargo clean --quiet
    Set-Location ..
    Set-Location ..
    Set-Location btils
    make clean --quiet
    Set-Location ..
    Set-Location site
    Remove-Item -Path "node_modules" -Recurse -Force -ErrorAction SilentlyContinue
    Print-Message "Cleaned up!" "green"
}

function Check-Deps {
    if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
        Print-Message "Cargo is not installed. Would you like to install it? [y/N]" "yellow"
        
        $userInput = Read-Host -Prompt ""

        if ($userInput -eq 'y' -or $userInput -eq 'Y') {
            Print-Message "Installing Cargo..." "yellow"
            Invoke-WebRequest -Uri https://win.rustup.rs -OutFile "rustup-init.exe"
            Start-Process -FilePath ".\rustup-init.exe" -ArgumentList "-y" -NoNewWindow -Wait
            Remove-Item -Path "rustup-init.exe" -Force
            $env:Path += ";$HOME\.cargo\bin"
            Print-Message "Cargo installed successfully!" "green"
        } else {
            Print-Message "Cargo installation skipped." "red"
            exit
        }
    }
    if (-not (Get-Command gcc -ErrorAction SilentlyContinue)) {
	Print-Message "GCC is not installed. Would you like to install GCC and Make? [y/N]" "yellow"
	
	$userInput = Read-Host -Prompt ""
	
	    if ($userInput -eq 'y' -or $userInput -eq 'Y') {
	        Print-Message "Installing..." "yellow"
	        Set-ExecutionPolicy Bypass -Scope Process -Force; [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072; iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))
            $env:PATH += ";C:\tools\mingw\bin"
	        Print-Message "Make/GCC installed!" "green"
	    } else {
	        Print-Message "Make/GCC installation skipped." "red"
	        exit
	    }
    }
}

function Spinner {
    param (
        [int]$processId,
        [string]$message
    )
    $delay = 0.1
    Print-Message "$message" "blue"
    $i = 0

    while (Get-Process -Id $processId -ErrorAction SilentlyContinue) {
        Start-Sleep -Seconds $delay
        $i++
    }
    Clear-Line
    Print-Message "Done!" "green"
}

function Print-Help {
    param (
        [string]$ScriptName
    )

    Write-Host "The build script for the BELLE programs and utilities`n"
    Write-Host "`e[4mUsage`e[0m: $ScriptName [OPTIONS] [TARGETS]"
    Write-Host "Options:"
    Write-Host "  -c, --clean        Clean the build directories (doesn't build)"
    Write-Host "  -w, --with-cleanup Clean directories after building"
    Write-Host "  -q, --quiet        Suppress output"
    Write-Host "  -h, --help         Display this help message"
    Write-Host "`nTargets:"
    Write-Host "  bdump, basm, belle, bfmt (default: all)"
    exit
}

function Default-Build {
    if (-not (Test-Path bin)) {
        New-Item -ItemType Directory -Path bin
    }

    if ($Clean) {
        Clean
        exit
    }

    foreach ($Target in $Targets) {
        switch ($Target) {
            "basm" {
                Set-Location basm
                Start-Process -FilePath "cargo" -ArgumentList "build", "--release", "--quiet" -NoNewWindow -PassThru | ForEach-Object {
                    $PPid = $_.Id
                    Spinner $PPid "Building BELLE-asm..."
                    Copy-Item -Path "target\release\basm.exe" -Destination "../bin" -Force
                }
                Set-Location ..
            }
            "bdump" {
                Set-Location bdump
                Start-Process -FilePath "make" -ArgumentList "--quiet" -NoNewWindow -PassThru | ForEach-Object {
                    $PPid = $_.Id
                    Spinner $PPid "Building BELLE-dump..."
                    Copy-Item -Path "bdump.exe" -Destination "../bin" -Force
                }
                Set-Location ..
            }
            "belle" {
                Set-Location belle
                Start-Process -FilePath "cargo" -ArgumentList "build", "--release", "--quiet" -NoNewWindow -PassThru | ForEach-Object {
                    $PPid = $_.Id
                    Spinner $PPid "Building BELLE..."
                    Copy-Item -Path "target\release\belle.exe" -Destination "../bin" -Force
                }
                Set-Location ..
            }
            "bfmt" {
                Set-Location btils
                Start-Process -FilePath "make" -ArgumentList "--quiet" -NoNewWindow -PassThru | ForEach-Object {
                    $PPid = $_.Id
                    Spinner $PPid "Building BELLE-fmt..."
                    Copy-Item -Path "bfmt.exe" -Destination "../bin" -Force
                }
                Set-Location ..
            }
        }
    }

    if ($WithCleanup) {
        Clean
    }

    Print-Message "Build complete" "green"
    exit
}

Check-Deps

$Targets = @()

foreach ($Arg in $args) {
    switch ($Arg) {
        "--clean" { $Clean = $true }
        "-c"      { $Clean = $true }
        "--with-cleanup" { $WithCleanup = $true }
        "-w"      { $WithCleanup = $true }
        "--quiet" { $Quiet = $true }
        "-q"      { $Quiet = $true }
        "--nospin"{ $Nospin = $true }
        "-n"      { $Nospin = $true }
        "--help"  { Print-Help $MyInvocation.MyCommand.Path }
        "-h"      { Print-Help $MyInvocation.MyCommand.Path }
        "bdump"   { $Targets += "bdump" }
        "basm"    { $Targets += "basm" }
        "belle"   { $Targets += "belle" }
        "bfmt"    { $Targets += "bfmt" }
    }
}

if ($Targets.Count -eq 0) {
    $Targets += "bdump", "basm", "belle", "bfmt"
}

Default-Build
