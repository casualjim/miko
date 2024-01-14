SELECT
  chats.id,
  title,
  chats.user_id,
  users.email AS user_email,
  chats.created_at,
  chats.updated_at
FROM
  chats
  INNER JOIN users ON users.id = chats.user_id
WHERE
  user_id = $1
ORDER BY
  chats.updated_at DESC
