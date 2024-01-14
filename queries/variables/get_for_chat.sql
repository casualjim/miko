SELECT
  v.id,
  v.key,
  v.value,
  v.chat_id,
  c.user_id,
  v.created_at
FROM
  variables v
  INNER JOIN chats c ON v.chat_id = c.id
WHERE
  v.chat_id = $1
ORDER BY
  v.created_at DESC
