CREATE TYPE status_type AS ENUM ('INITIAL', 'SELLER-ACCEPTED', 'DRIVER-TO-BOOTH', 'DELIVERD', 'IN-TRANSIT' ,'AUTO-REFUND', 'BUYER-REFUND', 'SELLER-REFUND', 'COMPLETED');
CREATE TYPE order_type AS ENUM ('WTL-DELIVERY', 'BOOTH-DELIVERY', 'SHIPPING', 'PICKUP');

CREATE TABLE
  orders (
	id UUID PRIMARY KEY, 
  account_id UUID REFERENCES accounts(id),
  booth_id UUID REFERENCES booths(id),
	status status_type,
	order_type order_type,
	multisig_address varchar(255),
	redeem_script varchar(255),
	signatures TEXT[],
  sub_total DOUBLE PRECISION,
  delivery_fee DOUBLE PRECISION,
  shipping_fee DOUBLE PRECISION,
  escrow_fee DOUBLE PRECISION,
  total DOUBLE PRECISION,
  created_at TIMESTAMPTZ,
  updated_at TIMESTAMPTZ
);



CREATE TABLE
  order_to_bid (
	id UUID PRIMARY KEY, 
  account_id UUID REFERENCES accounts(id),
  order_id UUID REFERENCES orders(id),
  bid_id UUID REFERENCES bids(id),
  created_at TIMESTAMPTZ,
  updated_at TIMESTAMPTZ
);

CREATE TABLE
  order_to_product (
	id UUID PRIMARY KEY, 
  account_id UUID REFERENCES accounts(id),
  order_id UUID REFERENCES orders(id),
  product_id UUID REFERENCES products(id),
  created_at TIMESTAMPTZ,
  updated_at TIMESTAMPTZ
);


CREATE TABLE
	order_wtl_delivery_info (
	id UUID PRIMARY KEY, 
  order_id UUID REFERENCES orders(id),
  driver_id UUID REFERENCES drivers(id),
  delivery_address UUID REFERENCES address(id),
  booth_address UUID REFERENCES address(id),
  delivery_fee DOUBLE PRECISION,
  wtl_delivery_type wtl_delivery_type,
  created_at TIMESTAMPTZ,
  updated_at TIMESTAMPTZ
);


CREATE TABLE
	order_self_delivery_info (
	id UUID PRIMARY KEY, 
  order_id UUID REFERENCES carts(id),
  delivery_address UUID REFERENCES address(id),
  delivery_fee DOUBLE PRECISION,
  created_at TIMESTAMPTZ,
  updated_at TIMESTAMPTZ
);


CREATE TABLE
	order_shipping_info (
	id UUID PRIMARY KEY, 
  order_id UUID REFERENCES orders(id),
  shipping_address UUID REFERENCES address(id),
  shipping_fee DOUBLE PRECISION,
  shipping_type shipping_type,
  created_at TIMESTAMPTZ,
  updated_at TIMESTAMPTZ
);


CREATE TABLE
	order_pickup_info (
	id UUID PRIMARY KEY, 
  order_id UUID REFERENCES orders(id),
  pickup_address UUID REFERENCES address(id),
  created_at TIMESTAMPTZ,
  updated_at TIMESTAMPTZ
);
