$ErrorActionPreference = 'Stop'
$installer = Get-ChildItem 'src-tauri/target/release/bundle/nsis/*.exe' | Select-Object -First 1
if (!$installer -or $installer.Length -eq 0) { throw 'Installer missing or empty' }
$installDir = Join-Path $env:RUNNER_TEMP 'HOG4Pilot-Smoke'
$setup = Start-Process -FilePath $installer.FullName -ArgumentList '/S', "/D=$installDir" -Wait -PassThru
if ($setup.ExitCode -ne 0) { throw "Installer failed: $($setup.ExitCode)" }
$exe = Get-ChildItem $installDir -Filter '*.exe' | Where-Object { $_.Name -notmatch 'uninstall' } | Select-Object -First 1
if (!$exe) { throw 'Installed application missing' }
$app = Start-Process -FilePath $exe.FullName -PassThru
try {
  Start-Sleep -Seconds 10
  $app.Refresh()
  if ($app.HasExited) { throw "Application exited early: $($app.ExitCode)" }
  if ($app.MainWindowHandle -eq 0) { throw 'No application window detected' }
  Write-Output 'NSIS installation and application window launch passed; IPC workflows require separate E2E tests.'
} finally {
  if (!$app.HasExited) { Stop-Process -Id $app.Id }
}
