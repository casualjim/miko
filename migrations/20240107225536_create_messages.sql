-- Add migration script here
create table if not exists messages (
  id uuid default gen_random_uuid() primary key,
  chat_id uuid not null references chats(id) on delete cascade,
  role text not null,
  content text,
  name text,
  function_call jsonb,
  temporary boolean not null default false,
  tool_call_id text,
  tool_calls jsonb,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now()
);