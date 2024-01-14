SELECT
  m.id,
  m.chat_id,
  c.user_id,
  m.content,
  m.name,
  m.tool_calls,
  m.temporary,
  m.role,
  m.tool_call_id,
  m.created_at
FROM
  messages m
  INNER JOIN chats c ON c.id = m.chat_id
WHERE
  chat_id = $1
ORDER BY
  created_at DESC
