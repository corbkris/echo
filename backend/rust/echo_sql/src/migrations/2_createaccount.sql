CREATE TYPE account_type AS ENUM ('basic', 'managed');

CREATE TABLE
  accounts (
    id UUID PRIMARY KEY,
    username varchar(255) UNIQUE NOT NULL,
    password varchar(255) NOT NULL,
    days_active int,
    verified boolean,
    account_type account_type NOT NULL,
    created_at TIMESTAMPTZ,
    updated_at TIMESTAMPTZ
  );
