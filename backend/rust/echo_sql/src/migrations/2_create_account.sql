CREATE TABLE
  accounts (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    username varchar(255) UNIQUE NOT NULL,
    password varchar(255) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
  );
