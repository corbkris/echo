CREATE TABLE
  accounts (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    username varchar(255) UNIQUE NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
  );

CREATE INDEX idx_accounts_username ON accounts(username);
CREATE INDEX idx_accounts_created_at ON accounts(created_at);


CREATE TYPE account_type AS ENUM ('basic', 'managed');

CREATE TABLE account_info (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    account_id UUID REFERENCES accounts(id) UNIQUE,
		password varchar(255) NOT NULL,
    account_type account_type NOT NULL,
    days_active int,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE managed_account_info (
    id UUID PRIMARY KEY REFERENCES account_info(id),
    email varchar(255) UNIQUE NOT NULL,
    phone varchar(255) UNIQUE,
		verified BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_managed_account_info_email ON managed_account_info(email);
CREATE INDEX idx_managed_account_info_phone ON managed_account_info(phone);

CREATE TABLE basic_account_info (
    id UUID PRIMARY KEY REFERENCES account_info(id),
    recovery_key UUID UNIQUE NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);


CREATE TYPE address_type AS ENUM ('ACCOUNT', 'BOOTH', 'DRIVER');

CREATE TABLE address (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    account_id UUID REFERENCES accounts(id),
		address_type address_type,
		first_line varchar(255) NOT NULL,
		second_line varchar(255) NOT NULL,
		state varchar(255) NOT NULL,
		city varchar(255) NOT NULL,
		zip varchar(255) NOT NULL,
		country varchar(255) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE account_to_address (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    account_id UUID REFERENCES accounts(id) UNIQUE,
    address_id UUID REFERENCES address(id) UNIQUE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);


CREATE TYPE crypto_type AS ENUM ('BTC', 'XRP', 'XMR');
CREATE TYPE crypto_info_type AS ENUM ('ACCOUNT', 'BOOTH', 'DRIVER');

CREATE TABLE crypto_info (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    account_id UUID REFERENCES accounts(id),
		crypto_info_type crypto_info_type,
		address varchar(255) UNIQUE NOT NULL,
		public_key varchar(255) UNIQUE NOT NULL,
    crypto_type crypto_type NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE account_to_crypto_info (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    account_id UUID REFERENCES accounts(id) UNIQUE,
    crypto_info_id UUID REFERENCES crypto_info(id) UNIQUE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
