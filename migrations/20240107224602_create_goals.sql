-- Add migration script here
create table if not exists goals (
  id uuid default gen_random_uuid() primary key,
  chat_id uuid not null references chats(id) on delete cascade,
  user_id uuid not null references users(id) on delete cascade,
  prompt text not null,
  submission_date timestamptz not null,
  subsidized boolean not null default false,
  subsidized_completion_req numeric default 0 not null,
  subsidized_embedding_req numeric default 0 not null,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now()
);
create table if not exists goals_duplicate (
  id uuid default gen_random_uuid() primary key,
  chat_id uuid not null references chats(id) on delete cascade,
  user_id uuid not null references users(id) on delete cascade,
  prompt text not null,
  submission_date timestamptz not null,
  subsidized boolean not null default false,
  subsidized_completion_req numeric default 0 not null,
  subsidized_embedding_req numeric default 0 not null,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now()
);