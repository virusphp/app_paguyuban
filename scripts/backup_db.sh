#!/usr/bin/env bash
# ==============================================================================
# Skrip Backup Otomatis Database PostgreSQL — Aplikasi Manajemen Paguyuban
# ==============================================================================
# Sesuai spesifikasi PLAN.md Bagian 10 (Keamanan & Privasi):
# "Backup pg_dump harian, simpan di luar server, uji pemulihan tiap kuartal."
# ==============================================================================

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
BACKUP_DIR="${PROJECT_ROOT}/backups"
RETENTION_DAYS=30
TIMESTAMP="$(date +"%Y%m%d_%H%M%S")"
BACKUP_FILE="${BACKUP_DIR}/backup_paguyuban_${TIMESTAMP}.sql.gz"

mkdir -p "${BACKUP_DIR}"

# Muat variabel lingkungan dari .env jika ada
if [ -f "${PROJECT_ROOT}/.env" ]; then
    # shellcheck disable=SC2046
    export $(grep -v '^#' "${PROJECT_ROOT}/.env" | grep -v '^$' | xargs)
fi

DB_HOST="${DB_HOST:-127.0.0.1}"
DB_PORT="${DB_PORT:-5432}"
DB_NAME="${DB_DATABASE:-db_paguyuban}"
DB_USER="${DB_USERNAME:-postgres}"
export PGPASSWORD="${DB_PASSWORD:-postgres}"

echo "=========================================================="
echo " [PAGUYUBAN] Memulai Backup Database PostgreSQL"
echo " Waktu: $(date)"
echo " Target: ${BACKUP_FILE}"
echo "=========================================================="

# Metode 1: Gunakan pg_dump jika terinstal langsung di host
if command -v pg_dump >/dev/null 2>&1; then
    echo ">> Menjalankan pg_dump dari host..."
    pg_dump -h "${DB_HOST}" -p "${DB_PORT}" -U "${DB_USER}" -d "${DB_NAME}" --clean --if-exists | gzip > "${BACKUP_FILE}"
# Metode 2: Gunakan Docker exec jika pg_dump ada di dalam container
elif command -v docker >/dev/null 2>&1 && docker ps --format '{{.Names}}' | grep -q "paguyuban_postgres"; then
    echo ">> pg_dump lokal tidak ditemukan. Menjalankan pg_dump via docker exec..."
    docker exec -e PGPASSWORD="${PGPASSWORD}" paguyuban_postgres pg_dump -U "${DB_USER}" -d "${DB_NAME}" --clean --if-exists | gzip > "${BACKUP_FILE}"
else
    echo "ERROR: Tidak ditemukan pg_dump lokal maupun kontainer docker 'paguyuban_postgres'." >&2
    exit 1
fi

FILE_SIZE=$(du -h "${BACKUP_FILE}" | cut -f1)
echo ">> Backup BERHASIL dibuat: ${BACKUP_FILE} (${FILE_SIZE})"

# Rotasi Retensi: Hapus arsip backup yang berusia lebih dari RETENTION_DAYS hari
echo ">> Menjalankan rotasi pembersihan backup lebih dari ${RETENTION_DAYS} hari..."
find "${BACKUP_DIR}" -type f -name "backup_paguyuban_*.sql.gz" -mtime +"${RETENTION_DAYS}" -exec rm -f {} +
echo ">> Selesai rotasi."
echo "=========================================================="

