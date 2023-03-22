CREATE TABLE IF NOT EXISTS "telegram_media_group" (
    "group_id"   TEXT    NOT NULL,
    "msg_id"     INTEGER NOT NULL,
    "media_json" TEXT    NOT NULL,

    UNIQUE("group_id", "msg_id") ON CONFLICT REPLACE
);
