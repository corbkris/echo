CREATE TABLE
	checkout (
	id UUID PRIMARY KEY, 
  cart_id UUID REFERENCES carts(id) UNIQUE,
	multisig_address varchar(255),
	refund_address varchar(255),
	paid BOOLEAN DEFAULT FALSE,
	refunded BOOLEAN DEFAULT FALSE,
	confirm BOOLEAN DEFAULT FALSE,
  created_at TIMESTAMPTZ,
  updated_at TIMESTAMPTZ
);
