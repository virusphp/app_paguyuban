use chrono::{Datelike, Local, NaiveDate};
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
struct FeePlanRow {
    id: Uuid,
    amount_idr: i64,
    active_from: NaiveDate,
    active_to: Option<NaiveDate>,
}

#[derive(Debug, sqlx::FromRow)]
struct MemberRow {
    id: Uuid,
    joined_on: NaiveDate,
}

pub struct FeeService;

impl FeeService {
    /// Sinkronkan tagihan bulanan untuk 1 orang warga aktif
    /// Mulai dari bulan bergabung / tanggal aktif tarif hingga bulan berjalan saat ini
    pub async fn sync_invoices_for_member(
        pool: &sqlx::PgPool,
        org_id: Uuid,
        member_id: Uuid,
        joined_on: NaiveDate,
    ) -> Result<usize, sqlx::Error> {
        // Ambil semua tarif bulanan yang aktif untuk organisasi ini
        let plans: Vec<FeePlanRow> = sqlx::query_as(
            r#"
            SELECT id, amount_idr, active_from, active_to
            FROM fee_plans
            WHERE org_id = $1 AND period = 'bulanan'
            ORDER BY active_from ASC
            "#,
        )
        .bind(org_id)
        .fetch_all(pool)
        .await?;

        if plans.is_empty() {
            return Ok(0);
        }

        let now = Local::now().date_naive();
        let now_month_first = match NaiveDate::from_ymd_opt(now.year(), now.month(), 1) {
            Some(d) => d,
            None => return Ok(0),
        };

        let mut total_created = 0;

        for plan in plans {
            // Tentukan awal bulan tagihan:
            // Tanggal 1 dari bulan mulainya tarif atau bulan bergabung warga (ambil yang lebih baru / sesuai)
            let plan_start_month =
                NaiveDate::from_ymd_opt(plan.active_from.year(), plan.active_from.month(), 1)
                    .unwrap_or(plan.active_from);

            let member_start_month =
                NaiveDate::from_ymd_opt(joined_on.year(), joined_on.month(), 1)
                    .unwrap_or(joined_on);

            // Warga mulai ditagih dari mana yang lebih baru antara awal tarif atau awal bergabung warga
            let start_date = if member_start_month > plan_start_month {
                member_start_month
            } else {
                plan_start_month
            };

            // Batas akhir penerbitan: bulan berjalan saat ini (atau active_to jika tarif sudah usang)
            let mut end_date = now_month_first;
            if let Some(active_to) = plan.active_to {
                let plan_end_month =
                    NaiveDate::from_ymd_opt(active_to.year(), active_to.month(), 1)
                        .unwrap_or(active_to);
                if plan_end_month < end_date {
                    end_date = plan_end_month;
                }
            }

            if start_date > end_date {
                continue;
            }

            let mut cur = start_date;
            while cur <= end_date {
                let inv_id = Uuid::new_v4();
                let res = sqlx::query(
                    r#"
                    INSERT INTO invoices (id, org_id, member_id, fee_plan_id, period_month, amount_idr, status)
                    VALUES ($1, $2, $3, $4, $5, $6, 'belum')
                    ON CONFLICT (member_id, fee_plan_id, period_month) DO NOTHING
                    "#,
                )
                .bind(inv_id)
                .bind(org_id)
                .bind(member_id)
                .bind(plan.id)
                .bind(cur)
                .bind(plan.amount_idr)
                .execute(pool)
                .await?;

                if res.rows_affected() > 0 {
                    total_created += 1;
                }

                // Maju 1 bulan berikutnya
                cur = if cur.month() == 12 {
                    match NaiveDate::from_ymd_opt(cur.year() + 1, 1, 1) {
                        Some(d) => d,
                        None => break,
                    }
                } else {
                    match NaiveDate::from_ymd_opt(cur.year(), cur.month() + 1, 1) {
                        Some(d) => d,
                        None => break,
                    }
                };
            }
        }

        Ok(total_created)
    }

    /// Sinkronkan tagihan bulanan untuk SELURUH warga aktif dalam satu organisasi
    /// Memastikan semua warga memiliki tagihan dari awal tahun/awal tarif hingga bulan ini
    pub async fn sync_invoices_for_all_active_members(
        pool: &sqlx::PgPool,
        org_id: Uuid,
    ) -> Result<usize, sqlx::Error> {
        let members: Vec<MemberRow> = sqlx::query_as(
            r#"
            SELECT id, joined_on
            FROM members
            WHERE org_id = $1 AND status = 'aktif'
            "#,
        )
        .bind(org_id)
        .fetch_all(pool)
        .await?;

        let mut total_created = 0;
        for m in members {
            if let Ok(count) = Self::sync_invoices_for_member(pool, org_id, m.id, m.joined_on).await
            {
                total_created += count;
            }
        }

        Ok(total_created)
    }

    /// Terbitkan tagihan secara batch untuk rentang bulan tertentu (misal: 1..end_month)
    pub async fn generate_invoices_batch(
        pool: &sqlx::PgPool,
        org_id: Uuid,
        fee_plan_id: Uuid,
        year: i32,
        start_month: u32,
        end_month: u32,
    ) -> Result<usize, sqlx::Error> {
        #[derive(sqlx::FromRow)]
        struct PlanData {
            amount_idr: i64,
        }

        let plan: PlanData =
            sqlx::query_as("SELECT amount_idr FROM fee_plans WHERE id = $1 AND org_id = $2")
                .bind(fee_plan_id)
                .bind(org_id)
                .fetch_one(pool)
                .await?;

        let active_member_ids: Vec<Uuid> =
            sqlx::query_scalar("SELECT id FROM members WHERE org_id = $1 AND status = 'aktif'")
                .bind(org_id)
                .fetch_all(pool)
                .await?;

        let mut inserted_count = 0;

        for m in start_month..=end_month {
            let period_month = match NaiveDate::from_ymd_opt(year, m, 1) {
                Some(d) => d,
                None => continue,
            };

            for member_id in &active_member_ids {
                let inv_id = Uuid::new_v4();
                let res = sqlx::query(
                    r#"
                    INSERT INTO invoices (id, org_id, member_id, fee_plan_id, period_month, amount_idr, status)
                    VALUES ($1, $2, $3, $4, $5, $6, 'belum')
                    ON CONFLICT (member_id, fee_plan_id, period_month) DO NOTHING
                    "#,
                )
                .bind(inv_id)
                .bind(org_id)
                .bind(*member_id)
                .bind(fee_plan_id)
                .bind(period_month)
                .bind(plan.amount_idr)
                .execute(pool)
                .await?;

                if res.rows_affected() > 0 {
                    inserted_count += 1;
                }
            }
        }

        Ok(inserted_count)
    }
}
