CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE profiles (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  email TEXT UNIQUE NOT NULL,
  role TEXT CHECK (role IN ('renter', 'owner', 'admin')) DEFAULT 'renter',
  stripe_account_id TEXT UNIQUE,
  stripe_onboarding_complete BOOLEAN DEFAULT FALSE,
  created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE vehicles (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  owner_id UUID REFERENCES profiles(id) ON DELETE CASCADE NOT NULL,
  title TEXT NOT NULL,
  description TEXT,
  vehicle_type TEXT CHECK (vehicle_type IN ('class_a','class_b','class_c','trailer','truck_camper','popup')) NOT NULL,
  year INT,
  make TEXT,
  model TEXT,
  sleeps INT NOT NULL,
  length_ft INT,
  city TEXT,
  state TEXT,
  nightly_rate BIGINT NOT NULL,
  cleaning_fee BIGINT DEFAULT 0,
  min_nights INT DEFAULT 1,
  photos TEXT[] DEFAULT ARRAY[]::TEXT[],
  amenities TEXT[] DEFAULT ARRAY[]::TEXT[],
  status TEXT CHECK (status IN ('draft','active','paused')) DEFAULT 'draft',
  created_at TIMESTAMPTZ DEFAULT NOW()
);
CREATE INDEX idx_vehicles_owner ON vehicles(owner_id);
CREATE INDEX idx_vehicles_active ON vehicles(status) WHERE status = 'active';

CREATE TABLE availability (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  vehicle_id UUID REFERENCES vehicles(id) ON DELETE CASCADE NOT NULL,
  date DATE NOT NULL,
  is_reserved BOOLEAN DEFAULT FALSE,
  booking_id UUID,
  UNIQUE(vehicle_id, date)
);
CREATE INDEX idx_availability_lookup ON availability(vehicle_id, date, is_reserved) WHERE is_reserved = FALSE;

CREATE TABLE bookings (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  vehicle_id UUID REFERENCES vehicles(id) NOT NULL,
  renter_id UUID REFERENCES profiles(id) NOT NULL,
  start_date DATE NOT NULL,
  end_date DATE NOT NULL,
  total_price BIGINT NOT NULL,
  platform_fee BIGINT NOT NULL,
  status TEXT CHECK (status IN ('pending','paid','confirmed','cancelled','refunded')) DEFAULT 'pending',
  stripe_session_id TEXT,
  stripe_payment_intent_id TEXT,
  payout_transfer_id TEXT,
  payout_status TEXT CHECK (payout_status IN ('pending','processing','completed','failed')) DEFAULT 'pending',
  created_at TIMESTAMPTZ DEFAULT NOW()
);
CREATE UNIQUE INDEX idx_booking_no_overlap ON bookings(vehicle_id, start_date, end_date)
  WHERE status NOT IN ('cancelled', 'refunded');
CREATE INDEX idx_bookings_renter ON bookings(renter_id);
CREATE INDEX idx_bookings_payout ON bookings(payout_status) WHERE payout_status != 'completed';

INSERT INTO profiles (id, email, role) VALUES
  ('11111111-1111-1111-1111-111111111111', 'owner@test.wheelbase.dev', 'owner'),
  ('22222222-2222-2222-2222-222222222222', 'renter@test.wheelbase.dev', 'renter');
