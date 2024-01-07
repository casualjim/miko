-- Add migration script here
create extension if not exists timescaledb cascade;
create table if not exists users (
  id uuid default gen_random_uuid() primary key,
  email text not null unique,
  name text not null,
  picture text,
  family_name text,
  given_name text,
  provider text not null,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now()
);