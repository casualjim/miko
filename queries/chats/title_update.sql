WITH updated AS (
  UPDATE
    chats
  SET
    title = $1
  WHERE
    id = $2
  RETURNING
    *
)
SELECT
  updated.*,
  users.email AS user_email
FROM
  updated
  INNER JOIN users ON updated.user_id = users.id;

