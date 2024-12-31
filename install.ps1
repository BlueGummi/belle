$cleanup = $false
Set-StrictMode -Version Latest

$DIR = "bin"
$FILE1 = "basm.exe"
$FILE2 = "bdump.exe"
$FILE3 = "belle.exe"
$FILE4 = "bfmt.exe" 

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

function Install {
    Print-Message "Installing..." "blue"
    
    $localBinPath = "$HOME\.local\bin"
    
    if (-not (Test-Path $localBinPath)) {
        New-Item -ItemType Directory -Path $localBinPath -Force | Out-Null
    }

    $files = @($FILE1, $FILE2, $FILE3, $FILE4)
    foreach ($file in $files) {
        $sourcePath = Join-Path -Path $DIR -ChildPath $file
        if (Test-Path $sourcePath) {
            Copy-Item -Path $sourcePath -Destination $localBinPath -Force
	    $cutString = $file.Substring(0, $file.Length - 4)
            Print-Message "$cutString installed successfully." "green"
        } else {
            Print-Message "Warning: '$sourcePath' does not exist and cannot be copied." "yellow"
        }
    }

    Print-Message "Installation complete." "green"

    $pathEntries = $env:PATH -split ';'
    if (-not ($pathEntries -contains "$HOME\.local\bin")) {
        Print-Message "Updating PATH to include ~/.local/bin" "yellow"
        
        $profilePath = $PROFILE
        if (-not (Test-Path $profilePath)) {
            New-Item -ItemType File -Path $profilePath -Force | Out-Null
        }

        $profileContent = Get-Content -Path $profilePath -ErrorAction SilentlyContinue
        if (-not ($profileContent -match [regex]::Escape("$HOME\.local\bin"))) {
            Add-Content -Path $profilePath -Value 'if (-not ($env:PATH -split ";" | Where-Object { $_ -eq "$HOME\.local\bin" })) { $env:PATH += ";$HOME\.local\bin" }'
            Print-Message "Added '$HOME\.local\bin' to your PowerShell profile." "green"
        } else {
            Print-Message "'$HOME\.local\bin' is already in your PowerShell profile." "yellow"
        }
        
        Print-Message "Please restart your terminal or run '. $profilePath' to apply changes." "yellow"
    }

    if ($cleanup) {
        Print-Message "Deleting '$DIR'..." "yellow"
        Remove-Item -Path $DIR -Recurse -Force -ErrorAction SilentlyContinue
    }
}

function Print-Help {
    param (
        [string]$ScriptName
    )
    Write-Host "The install script for the BELLE programs and utilities`n"
    Write-Host "`e[4mUsage`e[0m: $ScriptName [OPTIONS]"
    Write-Host "Options:"
    Write-Host "  -c, --cleanup        Clean the binary directory"
    Write-Host "  -h, --help           Display this help message"
    exit 0
}

foreach ($arg in $args) {
    switch ($arg) {
        "--cleanup" { $cleanup = $true }
        "-c"        { $cleanup = $true }
        "--help"    { Print-Help $MyInvocation.MyCommand.Path }
        "-h"        { Print-Help $MyInvocation.MyCommand.Path }
    }
}

$BUILD = $false

if (-not (Test-Path $DIR)) {
    Print-Message "Directory '$DIR' does not exist." "red"
    $BUILD = $true
} else {
    $FILE1_PATH = "$DIR\$FILE1"
    $FILE2_PATH = "$DIR\$FILE2"
    $FILE3_PATH = "$DIR\$FILE3"
    $FILE4_PATH = "$DIR\$FILE4"

    if (-not (Test-Path $FILE1_PATH) -and -not (Test-Path $FILE2_PATH) -and -not (Test-Path $FILE3_PATH) -and -not (Test-Path $FILE4_PATH)) {
        Print-Message "All binaries '$FILE1', '$FILE2', '$FILE3', and '$FILE4' do not exist in '$DIR'." "red"
        $BUILD = $true
    } elseif (-not (Test-Path $FILE1_PATH)) {
        Print-Message "Binary '$FILE1' does not exist in '$DIR'." "red"
        $BUILD = $true
    } elseif (-not (Test-Path $FILE2_PATH)) {
        Print-Message "Binary '$FILE2' does not exist in '$DIR'." "red"
        $BUILD = $true
    } elseif (-not (Test-Path $FILE3_PATH)) {
        Print-Message "Binary '$FILE3' does not exist in '$DIR'." "red"
        $BUILD = $true
    } elseif (-not (Test-Path $FILE4_PATH)) {
        Print-Message "Binary '$FILE4' does not exist in '$DIR'." "red"
        $BUILD = $true
    }
}

if ($BUILD) {
    $ANSWER = Read-Host "Do you want to build BELLE to create the binaries? [Y/n]"
    $ANSWER = if ([string]::IsNullOrWhiteSpace($ANSWER)) { "Y" } else { $ANSWER }

    if ($ANSWER -match "^[Yy]$") {
        if (-not (Test-Path "./build.ps1")) {
            Print-Message "Build script 'build.ps1' not found." "red"
            exit 1
        }

        Print-Message "Building..." "blue"
        .\build.ps1
        Print-Message "Build successful. Proceeding to installation..." "green"
        Install
    } else {
        Print-Message "Exiting without installing." "yellow"
    }
} else {
    Install
}
