use rusqlite::{Connection, Result};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Bookmark {
    pub content_id: String,
    pub content: String,
    pub book_id: String,
    pub book_title: String,
    pub chapter_title: String,
    pub color: u8,
    pub chapter_progress: f64,
    pub create_date: String,
    pub write_date: String,
}

pub fn get_bookmarks(db: &Connection) -> Result<Vec<Bookmark>> {
    let mut stmt = db.prepare(
        "
SELECT
    bm.BookmarkID,
    bm.Text,
    bm.VolumeID,
    bm.Color,
    bm.ChapterProgress,
    bm.DateCreated,
    bm.DateModified,
    c.Title,
    c899.Title AS ChapterTitle
FROM Bookmark bm
LEFT JOIN content c ON c.ContentID = bm.VolumeID
LEFT JOIN content c9 ON c9.ContentID = CASE
    WHEN INSTR(bm.ContentID, '#') > 0
    THEN SUBSTR(bm.ContentID, 1, INSTR(bm.ContentID, '#') - 1)
    ELSE bm.ContentID
END AND c9.ContentType = 9
LEFT JOIN content c899 ON c899.BookID = c9.BookID
    AND c899.ContentType = 899
    AND (
        c899.ContentID = CONCAT(c9.ContentID, '-1')
        OR c899.ContentID LIKE CONCAT(c9.ContentID, '-%')
        OR c899.ContentID = CONCAT(bm.ContentID, '-1')
        OR c899.ContentID LIKE CONCAT(bm.ContentID, '-%')
    )
WHERE bm.Text IS NOT NULL AND bm.Text != ''
        ",
    )?;
    let bookmarks: Result<Vec<Bookmark>> = stmt
        .query_map([], |row| {
            Ok(Bookmark {
                content_id: row.get("BookmarkID")?,
                content: row.get("Text")?,
                book_id: row.get("VolumeID")?,
                book_title: row.get("Title")?,
                chapter_title: row.get("ChapterTitle")?,
                color: row.get("Color")?,
                chapter_progress: row.get("ChapterProgress")?,
                create_date: row.get("DateCreated")?,
                write_date: row.get("DateModified")?,
            })
        })?
        .collect();
    bookmarks
}
