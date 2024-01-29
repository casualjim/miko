SELECT
  l.id,
  l.chat_id,
  l."user",
  l.title,
  l.content,
  l.created_at
FROM
  logs l
WHERE
  l.chat_id = $1
ORDER BY
  l.created_at DESC
