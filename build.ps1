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
    Set-Location btils/bfmt
    make clean --quiet 
    Set-Location ..
    Set-Location ..
    Set-Location site
    Remove-Item -Path "node_modules" -Recurse -Force -ErrorAction SilentlyContinue
    Print-Message "Cleaned up!" "green"
}

function Spinner {
    param (
        [int]$processId,
        [string]$message
    )
    $delay = 0.1
    #$spin = '/-\|'
    Print-Message "$message" "blue"
    $i = 0
    
    while (Get-Process -Id $processId -ErrorAction SilentlyContinue) {
        #$temp = $spin[$i % $spin.Length]
        #Write-Host "`r$temp" -NoNewline
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
                Set-Location btils/bfmt
                Start-Process -FilePath "make" -ArgumentList "--quiet" -NoNewWindow -PassThru | ForEach-Object {
                    $PPid = $_.Id
                    Spinner $PPid "Building BELLE-fmt..."
                    Copy-Item -Path "bfmt.exe" -Destination "../../bin" -Force
                }
                Set-Location ..
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
