-- Add migration script here
create table if not exists prompts (
  id uuid default gen_random_uuid() primary key,
  user_id uuid not null references users(id) on delete cascade,
  submission_date date not null,
  prompt text not null,
  llm_requests bigint default 0 not null,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now()
);