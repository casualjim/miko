INSERT INTO chats(user_id)
  VALUES ($1)
RETURNING
  id, title, user_id, '', chats.created_at, chats.updated_at
