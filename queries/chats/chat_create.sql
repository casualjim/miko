WITH inserted AS (
INSERT INTO chats(id, user_id)
    VALUES ($1, $2)
  RETURNING
    *)
  SELECT
    inserted.*,
    users.email AS user_email
  FROM
    inserted
    INNER JOIN users ON inserted.user_id = users.id;

