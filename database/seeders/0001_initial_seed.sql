-- Seeder SQL Awal: Pengguna & Organisasi Contoh untuk Lingkungan Dev
-- Password default semua akun adalah: password123
-- Hash bcrypt default_cost: $2b$12$x4l3Z3lCsmBq95qOmsJ2eeUj557/j7d6s94mR76p/nFj8z1vE67G.

-- 1. Superadmin (Akses Penuh /sa/...)
INSERT INTO users (name, phone, password_hash, is_superadmin, status)
VALUES (
    'Super Admin Paguyuban',
    '081111111111',
    '$2b$12$I4D5U8UhPAQTvIDAzl38jO1cZXcCUrjY.ZDzV1Dm4QBQQDgnr4Mne',
    true,
    'aktif'
)
ON CONFLICT (phone) DO UPDATE 
    SET name = EXCLUDED.name, password_hash = EXCLUDED.password_hash;

-- 2. Organisasi 1: Paguyuban Kanfer 2
INSERT INTO organizations (id, name, type, status)
VALUES (
    'b5bac8a0-c424-428f-8c27-2534cb24e9f4',
    'Paguyuban Kanfer 2',
    'paguyuban',
    'aktif'
)
ON CONFLICT (id) DO NOTHING;

-- 3. Organisasi 2: RT 05 RW 09 Slamaran (Untuk Uji Isolasi Multi-Tenant)
INSERT INTO organizations (id, name, type, status)
VALUES (
    'bc729a29-e7c7-4988-8352-9be8ca59ca53',
    'RT 05 RW 09 Slamaran',
    'rt',
    'aktif'
)
ON CONFLICT (id) DO NOTHING;

-- 4. Pengurus Paguyuban Kanfer 2
-- Ketua (Admin)
INSERT INTO users (name, phone, password_hash, is_superadmin, status)
VALUES (
    'Bambang Sudarsono (Ketua)',
    '081234567890',
    '$2b$12$I4D5U8UhPAQTvIDAzl38jO1cZXcCUrjY.ZDzV1Dm4QBQQDgnr4Mne',
    false,
    'aktif'
)
ON CONFLICT (phone) DO UPDATE 
    SET name = EXCLUDED.name, password_hash = EXCLUDED.password_hash;

INSERT INTO memberships (org_id, user_id, role)
SELECT 
    'b5bac8a0-c424-428f-8c27-2534cb24e9f4'::uuid,
    u.id,
    'admin'::org_role
FROM users u WHERE u.phone = '081234567890'
ON CONFLICT (org_id, user_id) DO UPDATE SET role = EXCLUDED.role;

-- Bendahara
INSERT INTO users (name, phone, password_hash, is_superadmin, status)
VALUES (
    'Siti Rahayu (Bendahara)',
    '081234567891',
    '$2b$12$I4D5U8UhPAQTvIDAzl38jO1cZXcCUrjY.ZDzV1Dm4QBQQDgnr4Mne',
    false,
    'aktif'
)
ON CONFLICT (phone) DO UPDATE 
    SET name = EXCLUDED.name, password_hash = EXCLUDED.password_hash;

INSERT INTO memberships (org_id, user_id, role)
SELECT 
    'b5bac8a0-c424-428f-8c27-2534cb24e9f4'::uuid,
    u.id,
    'bendahara'::org_role
FROM users u WHERE u.phone = '081234567891'
ON CONFLICT (org_id, user_id) DO UPDATE SET role = EXCLUDED.role;

-- Sekretaris
INSERT INTO users (name, phone, password_hash, is_superadmin, status)
VALUES (
    'Ahmad Fauzi (Sekretaris)',
    '081234567892',
    '$2b$12$I4D5U8UhPAQTvIDAzl38jO1cZXcCUrjY.ZDzV1Dm4QBQQDgnr4Mne',
    false,
    'aktif'
)
ON CONFLICT (phone) DO UPDATE 
    SET name = EXCLUDED.name, password_hash = EXCLUDED.password_hash;

INSERT INTO memberships (org_id, user_id, role)
SELECT 
    'b5bac8a0-c424-428f-8c27-2534cb24e9f4'::uuid,
    u.id,
    'sekretaris'::org_role
FROM users u WHERE u.phone = '081234567892'
ON CONFLICT (org_id, user_id) DO UPDATE SET role = EXCLUDED.role;

-- 5. Pengurus RT 05 RW 09 (Organisasi Kedua)
INSERT INTO users (name, phone, password_hash, is_superadmin, status)
VALUES (
    'Hendra Gunawan (Ketua RT 05)',
    '081300000001',
    '$2b$12$I4D5U8UhPAQTvIDAzl38jO1cZXcCUrjY.ZDzV1Dm4QBQQDgnr4Mne',
    false,
    'aktif'
)
ON CONFLICT (phone) DO UPDATE 
    SET name = EXCLUDED.name, password_hash = EXCLUDED.password_hash;

INSERT INTO memberships (org_id, user_id, role)
SELECT 
    'bc729a29-e7c7-4988-8352-9be8ca59ca53'::uuid,
    u.id,
    'admin'::org_role
FROM users u WHERE u.phone = '081300000001'
ON CONFLICT (org_id, user_id) DO UPDATE SET role = EXCLUDED.role;
