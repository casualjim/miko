insert into users(
    email,
    name,
    picture,
    family_name,
    given_name,
    provider
  )
values ($1, $2, $3, $4, $5, 'zitadel') on conflict (email) do
update
set name = $2,
  picture = $3,
  family_name = $4,
  given_name = $5,
  updated_at = now()
returning email,
  family_name,
  given_name,
  name,
  picture;