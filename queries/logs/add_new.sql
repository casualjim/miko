INSERT INTO logs(chat_id, "user", title, content)
  VALUES ($1, $2, $3, $4)
RETURNING
  id, chat_id, "user", title, content, created_at
