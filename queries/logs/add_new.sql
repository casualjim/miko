INSERT INTO logs(chat_id, user_id, title, content)
  VALUES ($1, $2, $3, $4)
RETURNING
  id, chat_id, user_id, title, content, created_at
