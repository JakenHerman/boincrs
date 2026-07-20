<#
.SYNOPSIS
    Render and pack the boincrs Chocolatey package.

.DESCRIPTION
    Substitutes the {{VERSION}} and {{SHA256}} tokens in the nuspec and tools
    templates, writes the rendered package into packaging/chocolatey/out, and
    runs `choco pack`. The resulting .nupkg is written to the same out
    directory.

    Invoked from .github/workflows/release.yml, and runnable locally by a
    maintainer:

        pwsh packaging/chocolatey/build.ps1 -Version 1.0.0 -Sha256 <zip-sha256>

.PARAMETER Version
    Release version without the leading "v" (e.g. 1.0.0).

.PARAMETER Sha256
    Lowercase SHA256 of boincrs-<Version>-x86_64-pc-windows-msvc.zip.
#>
param(
    [Parameter(Mandatory = $true)][string]$Version,
    [Parameter(Mandatory = $true)][string]$Sha256
)

$ErrorActionPreference = 'Stop'

$root = $PSScriptRoot
$out  = Join-Path $root 'out'

if (Test-Path $out) { Remove-Item -Recurse -Force $out }
New-Item -ItemType Directory -Force -Path (Join-Path $out 'tools') | Out-Null

$sha = $Sha256.ToLower()

function Expand-Template {
    param([string]$InPath, [string]$OutPath)
    (Get-Content -Raw $InPath).
        Replace('{{VERSION}}', $Version).
        Replace('{{SHA256}}', $sha) |
        Set-Content -Encoding UTF8 $OutPath
}

Expand-Template (Join-Path $root 'boincrs.nuspec')              (Join-Path $out 'boincrs.nuspec')
Expand-Template (Join-Path $root 'tools/chocolateyinstall.ps1') (Join-Path $out 'tools/chocolateyinstall.ps1')
Expand-Template (Join-Path $root 'tools/VERIFICATION.txt')      (Join-Path $out 'tools/VERIFICATION.txt')

Copy-Item (Join-Path $root 'tools/chocolateyuninstall.ps1') (Join-Path $out 'tools/chocolateyuninstall.ps1')
Copy-Item (Join-Path $root 'tools/LICENSE.txt')             (Join-Path $out 'tools/LICENSE.txt')

Write-Host "Rendered Chocolatey package for boincrs $Version (sha256 $sha)"

choco pack (Join-Path $out 'boincrs.nuspec') --outputdirectory $out
if ($LASTEXITCODE -ne 0) { throw "choco pack failed with exit code $LASTEXITCODE" }

$nupkg = Get-ChildItem -Path $out -Filter '*.nupkg' | Select-Object -First 1
Write-Host "Built package: $($nupkg.FullName)"
