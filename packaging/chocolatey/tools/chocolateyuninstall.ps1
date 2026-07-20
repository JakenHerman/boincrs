$ErrorActionPreference = 'Stop'

# Install-ChocolateyZipPackage records the files it extracted; Chocolatey removes
# them and the generated shim automatically on uninstall. Nothing extra is
# required here, but the script is kept so the package has an explicit,
# reviewable uninstall entry point.
$packageName = 'boincrs'
Write-Host "$packageName has been uninstalled."
