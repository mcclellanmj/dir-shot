SELECT
  CASE WHEN newer.name_id IS NULL
    THEN
      'DELETED'
    ELSE
      'CHANGED'
    END status,
  name.file_name FROM file_snaps AS older
  LEFT JOIN file_snaps as newer ON newer.name_id = older.name_id AND newer.record_date = ?1
  LEFT JOIN file_names as name ON older.name_id = name.id
  WHERE older.record_date = ?2 AND (
    (older.size != newer.size OR older.modified != newer.modified) OR
    (newer.name_id IS NULL) )
UNION ALL
SELECT 'NEW' status, name.file_name FROM file_snaps AS newer
  LEFT JOIN file_snaps as older ON older.name_id = newer.name_id AND older.record_date = ?2
  LEFT JOIN file_names as name ON newer.name_id = name.id
  WHERE newer.record_date = ?1 AND older.modified IS NULL;