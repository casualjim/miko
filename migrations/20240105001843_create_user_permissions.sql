-- Add migration script here
create table if not exists user_permissions (
  user_id uuid not null references users(id) on delete cascade,
  token text not null
);