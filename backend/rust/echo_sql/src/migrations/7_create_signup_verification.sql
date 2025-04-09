CREATE TABLE signup_verification (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
	code varchar(255) NOT NULL,
	username varchar(255) UNIQUE NOT NULL,
	email varchar(255) UNIQUE NOT NULL,
	password varchar(255) NOT NULL,
	expiration TIMESTAMPTZ NOT NULL,
	created_at TIMESTAMPTZ DEFAULT NOW(),
	updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_signup_verification_expiration ON signup_verification(expiration);
CREATE INDEX idx_signup_verification_code ON signup_verification(code);
