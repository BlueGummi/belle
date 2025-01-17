$zipUrl = "https://github.com/BlueGummi/belle/releases/download/nightly/belle-nightly-windows-x86_64.zip"
$zipFile = "$env:TEMP\belle-nightly-windows-x86_64.zip"
$extractPath = "$env:TEMP\belle"
$localBinPath = "$env:USERPROFILE\.local\bin"
$exeFiles = @("belle.exe", "basm.exe", "bdump.exe", "bfmt.exe")

if (-not (Test-Path $localBinPath)) {
    New-Item -ItemType Directory -Path $localBinPath
}

Invoke-WebRequest -Uri $zipUrl -OutFile $zipFile

Expand-Archive -Path $zipFile -DestinationPath $extractPath -Force

foreach ($exe in $exeFiles) {
    $sourcePath = Join-Path -Path $extractPath -ChildPath "windows-bin\$exe"
    $destinationPath = Join-Path -Path $localBinPath -ChildPath $exe

    if (Test-Path $sourcePath) {
        Move-Item -Path $sourcePath -Destination $destinationPath -Force
    }
}

$pathEnv = [System.Environment]::GetEnvironmentVariable("PATH")
if ($pathEnv -notlike "*$localBinPath*") {
    [System.Environment]::SetEnvironmentVariable("PATH", "$pathEnv;$localBinPath", [System.EnvironmentVariableTarget]::User)
    Write-Host "Restart your terminal"
}

Remove-Item -Path $zipFile -Force

Remove-Item -Path (Join-Path -Path $extractPath -ChildPath "windows-bin\*.exe") -Force

Move-Item -Path "windows-bin/examples" -Destination "." -Recurse -Force

Remove-Item -Path "windows-bin" -Recurse -Force

Set-Location -Path "examples"

ls

Write-Host "Installed."
Write-Host "Run 'make' in this directory to compile examples programs and execute them"
