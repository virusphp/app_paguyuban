# ==============================================================================
# Skrip Pemulihan (Restore) Database PostgreSQL (PowerShell)
# Aplikasi Manajemen Paguyuban / RT
# ==============================================================================

param (
    [Parameter(Mandatory=$true)]
    [string]$BackupFile
)

$ErrorActionPreference = "Stop"

if (-not (Test-Path $BackupFile)) {
    Write-Error "ERROR: File backup '$BackupFile' tidak ditemukan!"
    exit 1
}

Write-Host "==========================================================" -ForegroundColor Red
Write-Host " [PERINGATAN] PEMULIHAN DATABASE PAGUYUBAN" -ForegroundColor Red
Write-Host " File Target: $BackupFile"
Write-Host "=========================================================="
$confirm = Read-Host "Apakah Anda yakin ingin memulihkan database? Seluruh data akan ditimpa! (y/N)"
if ($confirm -ne "y" -and $confirm -ne "Y") {
    Write-Host "Operasi pemulihan dibatalkan." -ForegroundColor Yellow
    exit 0
}

$DbHost = "127.0.0.1"
$DbPort = "5432"
$DbName = "db_paguyuban"
$DbUser = "postgres"
$env:PGPASSWORD = "postgres"

Write-Host ">> Menjalankan restore database..." -ForegroundColor Yellow

$psqlCmd = Get-Command psql -ErrorAction SilentlyContinue
if ($null -ne $psqlCmd) {
    Get-Content $BackupFile | & psql -h $DbHost -p $DbPort -U $DbUser -d $DbName
} else {
    $dockerCheck = docker ps --filter "name=paguyuban_postgres" --format "{{.Names}}" 2>$null
    if ($dockerCheck -match "paguyuban_postgres") {
        Get-Content $BackupFile | docker exec -i paguyuban_postgres psql -U $DbUser -d $DbName
    } else {
        Write-Error "psql lokal maupun container Docker tidak ditemukan."
    }
}

Write-Host ">> Pemulihan database BERHASIL selesai!" -ForegroundColor Green
Write-Host "=========================================================="

