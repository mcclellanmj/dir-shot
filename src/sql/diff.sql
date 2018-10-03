SELECT
  CASE WHEN newer.path IS NULL
    THEN
      'DELETED'
    ELSE
      'CHANGED'
    END status,
  older.path FROM file_snaps AS older
  LEFT JOIN file_snaps as newer ON newer.path = older.path AND newer.record_date = ?1
  WHERE older.record_date = ?2 AND (
    (older.size != newer.size OR older.modified != newer.modified) OR
    (newer.path IS NULL) )
UNION ALL
SELECT 'NEW' status, newer.path FROM file_snaps AS newer
  LEFT JOIN file_snaps as older ON older.path = newer.path AND older.record_date = ?2
  WHERE newer.record_date = ?1 AND older.modified IS NULL;