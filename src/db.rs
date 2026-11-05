use sqlx::PgPool;

pub async fn initialize_database(pool: &PgPool) -> Result<(), sqlx::Error> {
    let statements = [
        "CREATE TABLE IF NOT EXISTS menus (
            id SERIAL PRIMARY KEY,
            name TEXT NOT NULL,
            price NUMERIC(12, 2) NOT NULL,
            category TEXT NOT NULL
        )",
        "CREATE TABLE IF NOT EXISTS orders (
            id SERIAL PRIMARY KEY,
            customer_name TEXT NOT NULL,
            item_name TEXT NOT NULL,
            quantity INT NOT NULL,
            status TEXT NOT NULL
        )",
        r#"CREATE TABLE IF NOT EXISTS "user" (
            id SERIAL PRIMARY KEY,
            username TEXT NOT NULL UNIQUE,
            password TEXT NOT NULL,
            role TEXT NOT NULL DEFAULT 'pembeli'
                CONSTRAINT user_role_valid CHECK (role IN ('admin', 'kasir', 'pembeli'))
        )"#,
    ];

    for statement in statements {
        sqlx::query(statement).execute(pool).await?;
    }

    sqlx::query(r#"ALTER TABLE "user" ADD COLUMN IF NOT EXISTS password TEXT"#)
        .execute(pool)
        .await?;

    sqlx::query(
        r#"ALTER TABLE "user" ADD COLUMN IF NOT EXISTS role TEXT NOT NULL DEFAULT 'pembeli'"#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"DO $$ BEGIN
            IF NOT EXISTS (
                SELECT 1 FROM pg_constraint WHERE conname = 'user_role_valid'
            ) THEN
                ALTER TABLE "user" ADD CONSTRAINT user_role_valid
                    CHECK (role IN ('admin', 'kasir', 'pembeli'));
            END IF;
        END $$"#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"ALTER TABLE orders ADD COLUMN IF NOT EXISTS user_id INTEGER
            REFERENCES "user"(id) ON DELETE SET NULL"#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"ALTER TABLE orders ADD COLUMN IF NOT EXISTS menu_id INTEGER
            REFERENCES menus(id) ON DELETE SET NULL"#,
    )
    .execute(pool)
    .await?;

    sqlx::query("ALTER TABLE orders ADD COLUMN IF NOT EXISTS unit_price NUMERIC(12, 2)")
        .execute(pool)
        .await?;

    sqlx::query("ALTER TABLE menus ADD COLUMN IF NOT EXISTS stock INTEGER NOT NULL DEFAULT 0")
        .execute(pool)
        .await?;

    sqlx::query(
        "DO $$ BEGIN
            IF NOT EXISTS (SELECT 1 FROM pg_constraint WHERE conname = 'menus_stock_nonnegative') THEN
                ALTER TABLE menus ADD CONSTRAINT menus_stock_nonnegative CHECK (stock >= 0);
            END IF;
        END $$",
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"CREATE TABLE IF NOT EXISTS payments (
            id SERIAL PRIMARY KEY,
            order_id INTEGER NOT NULL UNIQUE REFERENCES orders(id) ON DELETE CASCADE,
            user_id INTEGER REFERENCES "user"(id) ON DELETE SET NULL,
            method TEXT NOT NULL CHECK (method IN ('cash', 'bank_transfer')),
            status TEXT NOT NULL DEFAULT 'pending'
                CHECK (status IN ('pending', 'paid', 'rejected')),
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            paid_at TIMESTAMPTZ
        )"#,
    )
    .execute(pool)
    .await?;

    sqlx::query("ALTER TABLE payments DROP CONSTRAINT IF EXISTS payments_status_check")
        .execute(pool)
        .await?;

    sqlx::query("UPDATE payments SET status = 'lunas' WHERE status = 'paid'")
        .execute(pool)
        .await?;

    sqlx::query(
        "DO $$ BEGIN
            IF NOT EXISTS (
                SELECT 1 FROM pg_constraint
                WHERE conname = 'payments_status_valid' AND conrelid = 'payments'::regclass
            ) THEN
                ALTER TABLE payments ADD CONSTRAINT payments_status_valid
                    CHECK (status IN ('pending', 'lunas', 'rejected'));
            END IF;
        END $$",
    )
    .execute(pool)
    .await?;

    sqlx::query(
        "ALTER TABLE menus ALTER COLUMN price TYPE NUMERIC(12, 2) USING price::NUMERIC(12, 2)",
    )
    .execute(pool)
    .await?;

    Ok(())
}
