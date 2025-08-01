CREATE TYPE cart_type AS ENUM ('WTL-DELIVERY', 'BOOTH-DELIVERY', 'SHIPPING', 'PICKUP');

CREATE TABLE
  carts (
	id UUID PRIMARY KEY, 
  account_id UUID REFERENCES accounts(id),
  booth_id UUID REFERENCES booths(id),
  cart_type cart_type,
  sub_total DOUBLE PRECISION,
  delivery_fee DOUBLE PRECISION,
  shipping_fee DOUBLE PRECISION,
  escrow_fee DOUBLE PRECISION,
  total DOUBLE PRECISION,
  visible BOOLEAN,
  created_at TIMESTAMPTZ,
  updated_at TIMESTAMPTZ
);


CREATE TABLE
  cart_to_bid (
	id UUID PRIMARY KEY, 
  account_id UUID REFERENCES accounts(id),
  cart_id UUID REFERENCES carts(id),
  bid_id UUID REFERENCES bids(id),
  created_at TIMESTAMPTZ,
  updated_at TIMESTAMPTZ
);

CREATE TABLE
  cart_to_product (
	id UUID PRIMARY KEY, 
  account_id UUID REFERENCES accounts(id),
  cart_id UUID REFERENCES carts(id),
  product_id UUID REFERENCES products(id),
  created_at TIMESTAMPTZ,
  updated_at TIMESTAMPTZ
);



CREATE TABLE
	cart_wtl_delivery_info (
	id UUID PRIMARY KEY, 
  cart_id UUID REFERENCES carts(id),
  driver_id UUID REFERENCES drivers(id),
	delivery_type delivery_type DEFAULT 'WTL',
  delivery_address UUID REFERENCES address(id),
  delivery_fee DOUBLE PRECISION,
  wtl_delivery_type wtl_delivery_type,
  created_at TIMESTAMPTZ,
  updated_at TIMESTAMPTZ
);


CREATE TABLE
	cart_self_delivery_info (
	id UUID PRIMARY KEY, 
  cart_id UUID REFERENCES carts(id),
	delivery_type delivery_type DEFAULT 'SELF',
  delivery_address UUID REFERENCES address(id),
  delivery_fee DOUBLE PRECISION,
  created_at TIMESTAMPTZ,
  updated_at TIMESTAMPTZ
);


CREATE TABLE
	cart_shipping_info (
	id UUID PRIMARY KEY, 
  cart_id UUID REFERENCES carts(id),
  shipping_address UUID REFERENCES address(id),
  shipping_fee DOUBLE PRECISION,
  shipping_type shipping_type,
  created_at TIMESTAMPTZ,
  updated_at TIMESTAMPTZ
);


CREATE TABLE
	cart_pickup_info (
	id UUID PRIMARY KEY, 
  cart_id UUID REFERENCES carts(id),
  pickup_address UUID REFERENCES address(id),
	delivery_type delivery_type DEFAULT 'PICKUP',
  created_at TIMESTAMPTZ,
  updated_at TIMESTAMPTZ
);
