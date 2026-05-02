use chrono::NaiveDate;
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

use crate::error::{AppError, Result};

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum BookingStatus {
    Pending,
    Paid,
    Confirmed,
    Cancelled,
    Refunded,
}

pub async fn create_booking(
    pool: &PgPool,
    vehicle_id: Uuid,
    renter_id: Uuid,
    start: NaiveDate,
    end: NaiveDate,
    nightly_rate_cents: i64,
) -> Result<(Uuid, i64, i64)> {
    if end <= start {
        return Err(AppError::Validation(
            "End date must be after start date".into(),
        ));
    }

    let mut tx: Transaction<Postgres> = pool.begin().await?;
    let nights = (end - start).num_days() as i64;
    if nights < 1 {
        return Err(AppError::Validation(
            "Booking must be at least 1 night".into(),
        ));
    }
    let total_cents = nightly_rate_cents * nights;
    let platform_fee_cents = (total_cents * 10) / 100;

    let _vehicle =
        sqlx::query("SELECT id FROM vehicles WHERE id = $1 AND status = 'active' FOR UPDATE")
            .bind(vehicle_id)
            .fetch_optional(&mut *tx)
            .await?
            .ok_or_else(|| AppError::NotFound("Vehicle not found or inactive".into()))?;

    let reserved_count: i64 = sqlx::query_scalar(
        r#"SELECT COUNT(*) FROM availability
           WHERE vehicle_id = $1
           AND date >= $2 AND date <= $3
           AND is_reserved = TRUE"#,
    )
    .bind(vehicle_id)
    .bind(start)
    .bind(end)
    .fetch_one(&mut *tx)
    .await?;

    if reserved_count > 0 {
        return Err(AppError::Conflict("Dates already booked".into()));
    }

    let booking_id = Uuid::new_v4();
    sqlx::query(
        r#"INSERT INTO availability (vehicle_id, date, is_reserved, booking_id)
           SELECT $1, d::date, TRUE, $2
           FROM generate_series($3::date, $4::date, '1 day'::interval) AS d"#,
    )
    .bind(vehicle_id)
    .bind(booking_id)
    .bind(start)
    .bind(end)
    .execute(&mut *tx)
    .await?;

    sqlx::query(
        r#"INSERT INTO bookings
           (id, vehicle_id, renter_id, start_date, end_date, total_price, platform_fee, status)
           VALUES ($1, $2, $3, $4, $5, $6, $7, 'pending')"#,
    )
    .bind(booking_id)
    .bind(vehicle_id)
    .bind(renter_id)
    .bind(start)
    .bind(end)
    .bind(total_cents)
    .bind(platform_fee_cents)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok((booking_id, total_cents, platform_fee_cents))
}

pub async fn mark_paid(pool: &PgPool, booking_id: Uuid) -> Result<()> {
    sqlx::query("UPDATE bookings SET status = 'paid' WHERE id = $1")
        .bind(booking_id)
        .execute(pool)
        .await?;
    Ok(())
}
