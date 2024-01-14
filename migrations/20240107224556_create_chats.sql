-- Add migration script here
create table if not exists chats (
  id uuid default gen_random_uuid() primary key,
  title text,
  user_id uuid not null references users(id) on delete cascade,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now()
);
create table if not exists logs (
  id uuid default gen_random_uuid() primary key,
  chat_id uuid not null references chats(id) on delete cascade,
  user_id uuid not null references users(id) on delete cascade,
  title text not null,
  content text,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now()
);
create table if not exists variables (
  id uuid default gen_random_uuid() primary key,
  chat_id uuid not null references chats(id) on delete cascade,
  key text not null,
  value text not null,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now()
);