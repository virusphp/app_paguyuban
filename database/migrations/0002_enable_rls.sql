-- Migrasi 0002: Row Level Security (RLS) untuk Isolasi Data Multi-Organisasi
-- Setiap query dari role aplikasi (non-owner/non-superuser) hanya dapat mengakses data dengan org_id yang cocok.

ALTER TABLE memberships ENABLE ROW LEVEL SECURITY;
ALTER TABLE households ENABLE ROW LEVEL SECURITY;
ALTER TABLE members ENABLE ROW LEVEL SECURITY;
ALTER TABLE fee_plans ENABLE ROW LEVEL SECURITY;
ALTER TABLE invoices ENABLE ROW LEVEL SECURITY;
ALTER TABLE payments ENABLE ROW LEVEL SECURITY;
ALTER TABLE events ENABLE ROW LEVEL SECURITY;
ALTER TABLE event_rsvps ENABLE ROW LEVEL SECURITY;
ALTER TABLE transactions ENABLE ROW LEVEL SECURITY;
ALTER TABLE in_kind_donations ENABLE ROW LEVEL SECURITY;
ALTER TABLE announcements ENABLE ROW LEVEL SECURITY;
ALTER TABLE letter_templates ENABLE ROW LEVEL SECURITY;
ALTER TABLE letters ENABLE ROW LEVEL SECURITY;
ALTER TABLE documents ENABLE ROW LEVEL SECURITY;

-- Kebijakan Isolasi Organisasi (current_setting 'app.org_id')
DROP POLICY IF EXISTS memberships_org_isolation ON memberships;
CREATE POLICY memberships_org_isolation ON memberships
    USING (org_id = NULLIF(current_setting('app.org_id', true), '')::uuid);

DROP POLICY IF EXISTS households_org_isolation ON households;
CREATE POLICY households_org_isolation ON households
    USING (org_id = NULLIF(current_setting('app.org_id', true), '')::uuid);

DROP POLICY IF EXISTS members_org_isolation ON members;
CREATE POLICY members_org_isolation ON members
    USING (org_id = NULLIF(current_setting('app.org_id', true), '')::uuid);

DROP POLICY IF EXISTS fee_plans_org_isolation ON fee_plans;
CREATE POLICY fee_plans_org_isolation ON fee_plans
    USING (org_id = NULLIF(current_setting('app.org_id', true), '')::uuid);

DROP POLICY IF EXISTS invoices_org_isolation ON invoices;
CREATE POLICY invoices_org_isolation ON invoices
    USING (org_id = NULLIF(current_setting('app.org_id', true), '')::uuid);

DROP POLICY IF EXISTS payments_org_isolation ON payments;
CREATE POLICY payments_org_isolation ON payments
    USING (org_id = NULLIF(current_setting('app.org_id', true), '')::uuid);

DROP POLICY IF EXISTS events_org_isolation ON events;
CREATE POLICY events_org_isolation ON events
    USING (org_id = NULLIF(current_setting('app.org_id', true), '')::uuid);

DROP POLICY IF EXISTS event_rsvps_org_isolation ON event_rsvps;
CREATE POLICY event_rsvps_org_isolation ON event_rsvps
    USING (org_id = NULLIF(current_setting('app.org_id', true), '')::uuid);

DROP POLICY IF EXISTS transactions_org_isolation ON transactions;
CREATE POLICY transactions_org_isolation ON transactions
    USING (org_id = NULLIF(current_setting('app.org_id', true), '')::uuid);

DROP POLICY IF EXISTS in_kind_donations_org_isolation ON in_kind_donations;
CREATE POLICY in_kind_donations_org_isolation ON in_kind_donations
    USING (org_id = NULLIF(current_setting('app.org_id', true), '')::uuid);

DROP POLICY IF EXISTS announcements_org_isolation ON announcements;
CREATE POLICY announcements_org_isolation ON announcements
    USING (org_id = NULLIF(current_setting('app.org_id', true), '')::uuid);

DROP POLICY IF EXISTS letter_templates_org_isolation ON letter_templates;
CREATE POLICY letter_templates_org_isolation ON letter_templates
    USING (org_id = NULLIF(current_setting('app.org_id', true), '')::uuid);

DROP POLICY IF EXISTS letters_org_isolation ON letters;
CREATE POLICY letters_org_isolation ON letters
    USING (org_id = NULLIF(current_setting('app.org_id', true), '')::uuid);

DROP POLICY IF EXISTS documents_org_isolation ON documents;
CREATE POLICY documents_org_isolation ON documents
    USING (org_id = NULLIF(current_setting('app.org_id', true), '')::uuid);
