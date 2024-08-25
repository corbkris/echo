CREATE TABLE
  accounts (
    id UUID PRIMARY KEY,
    email varchar(255) UNIQUE NOT NULL,
    password varchar(255) NOT NULL,
    days_active int,
    verified boolean,
    created_at TIMESTAMPTZ,
    updated_at TIMESTAMPTZ
  );
