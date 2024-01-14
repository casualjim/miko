INSERT INTO users(email, name, picture, family_name, given_name, provider)
  VALUES ($1, $2, $3, $4, $5, 'zitadel')
ON CONFLICT (email)
  DO UPDATE SET
    name = $2, picture = $3, family_name = $4, given_name = $5, updated_at = now()
  RETURNING
    email, family_name, given_name, name, picture;

