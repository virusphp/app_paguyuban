#!/usr/bin/env bash
# ==============================================================================
# Skrip Pemulihan (Restore) Database PostgreSQL — Aplikasi Manajemen Paguyuban
# ==============================================================================
# Sesuai PLAN.md Bagian 10: "Uji pemulihan tiap kuartal."
# Gunakan dengan hati-hati karena akan menimpa data yang ada di database target!
# ==============================================================================

set -euo pipefail

if [ "$#" -lt 1 ]; then
    echo "Penggunaan: $0 <path_ke_file_backup.sql[.gz]>"
    echo "Contoh: $0 backups/backup_paguyuban_20261106_120000.sql.gz"
    exit 1
fi

BACKUP_FILE="$1"

if [ ! -f "${BACKUP_FILE}" ]; then
    echo "ERROR: Berkas backup '${BACKUP_FILE}' tidak ditemukan!" >&2
    exit 1
fi

echo "=========================================================="
echo " [PERINGATAN] PEMULIHAN DATABASE PAGUYUBAN"
echo " Berkas Target: ${BACKUP_FILE}"
echo "=========================================================="
read -p "Apakah Anda yakin ingin memulihkan database? Semua data saat ini akan ditimpa! (y/N): " CONFIRM
if [[ "${CONFIRM}" != "y" && "${CONFIRM}" != "Y" ]]; then
    echo "Operasi pemulihan dibatalkan."
    exit 0
fi

DB_HOST="${DB_HOST:-127.0.0.1}"
DB_PORT="${DB_PORT:-5432}"
DB_NAME="${DB_DATABASE:-db_paguyuban}"
DB_USER="${DB_USERNAME:-postgres}"
export PGPASSWORD="${DB_PASSWORD:-postgres}"

echo ">> Memulai proses pemulihan ke database ${DB_NAME}..."

if [[ "${BACKUP_FILE}" == *.gz ]]; then
    if command -v psql >/dev/null 2>&1; then
        gunzip -c "${BACKUP_FILE}" | psql -h "${DB_HOST}" -p "${DB_PORT}" -U "${DB_USER}" -d "${DB_NAME}"
    else
        gunzip -c "${BACKUP_FILE}" | docker exec -i paguyuban_postgres psql -U "${DB_USER}" -d "${DB_NAME}"
    fi
else
    if command -v psql >/dev/null 2>&1; then
        psql -h "${DB_HOST}" -p "${DB_PORT}" -U "${DB_USER}" -d "${DB_NAME}" < "${BACKUP_FILE}"
    else
        docker exec -i paguyuban_postgres psql -U "${DB_USER}" -d "${DB_NAME}" < "${BACKUP_FILE}"
    fi
fi

echo ">> Pemulihan database BERHASIL selesai!"
echo "=========================================================="

