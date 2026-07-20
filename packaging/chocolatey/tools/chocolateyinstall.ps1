$ErrorActionPreference = 'Stop'

# The version and checksum below are filled in by packaging/chocolatey/build.ps1
# before the package is built. The checksum pins the exact release asset so
# Chocolatey refuses to install a tampered or mismatched download.
$packageName = 'boincrs'
$version     = '{{VERSION}}'
$toolsDir    = Split-Path -Parent $MyInvocation.MyCommand.Definition
$url64       = "https://github.com/jakenherman/boincrs/releases/download/v$version/boincrs-$version-x86_64-pc-windows-msvc.zip"

$packageArgs = @{
  PackageName    = $packageName
  UnzipLocation  = $toolsDir
  Url64bit       = $url64
  Checksum64     = '{{SHA256}}'
  ChecksumType64 = 'sha256'
}

# Downloads and extracts boincrs.exe into the package tools directory.
# Chocolatey automatically generates a `boincrs` shim on the PATH for the
# extracted executable.
Install-ChocolateyZipPackage @packageArgs
