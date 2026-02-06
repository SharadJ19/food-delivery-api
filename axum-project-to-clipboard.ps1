# Root of the Rust project (current directory)
$Root = Get-Location

# Helper: get relative path
function Get-RelativePath($fullPath) {
    return $fullPath.Replace($Root.Path + "\", "")
}

# Ordered file patterns (Axum/Rust specific)
$orderedFiles = @(
    "Cargo.toml",
    "Cargo.lock",
    "build.rs",
    "src/main.rs",
    "src/lib.rs",
    "src/app.rs",
    "src/router.rs",
    "src/state.rs"
)

$orderedGlobs = @(
    "src/handlers/**/*.rs",
    "src/routes/**/*.rs",
    "src/middleware/**/*.rs",
    "src/models/**/*.rs",
    "src/**/*.rs"
)

$seen = New-Object System.Collections.Generic.HashSet[string]
$output = New-Object System.Text.StringBuilder

# Add explicitly ordered files
foreach ($file in $orderedFiles) {
    $full = Join-Path $Root $file
    if (Test-Path $full) {
        $seen.Add((Resolve-Path $full).Path) | Out-Null
        $rel = Get-RelativePath (Resolve-Path $full).Path

        $comment = if ($file.EndsWith(".toml")) { "# $rel" } else { "// $rel" }

        $output.AppendLine($comment) | Out-Null
        $output.AppendLine((Get-Content $full -Raw)) | Out-Null
        $output.AppendLine("`n") | Out-Null
    }
}

# Add remaining Rust files in structured order
foreach ($glob in $orderedGlobs) {
    Get-ChildItem -Path $glob -Recurse -File -ErrorAction SilentlyContinue |
        Sort-Object FullName |
        ForEach-Object {
            $resolved = (Resolve-Path $_.FullName).Path
            if (-not $seen.Contains($resolved)) {
                $seen.Add($resolved) | Out-Null
                $rel = Get-RelativePath $resolved

                $output.AppendLine("// $rel") | Out-Null
                $output.AppendLine((Get-Content $_.FullName -Raw)) | Out-Null
                $output.AppendLine("`n") | Out-Null
            }
        }
}

# Copy final output to clipboard
$output.ToString() | Set-Clipboard
