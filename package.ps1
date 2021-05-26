cargo.exe build &&
Copy-Item .\appx\* .\target\debug &&
Push-Location .\target\debug &&
Import-Module -Name Appx -UseWindowsPowerShell &&
Add-AppxPackage -Register AppxManifest.xml &&
Pop-Location
