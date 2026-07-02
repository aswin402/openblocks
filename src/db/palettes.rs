use crate::error::Result;
use crate::models::palette::{NewPalette, Palette};
use chrono::Utc;
use rusqlite::{Connection, params};
use uuid::Uuid;

/// Insert a new palette into the database
pub fn insert_palette(conn: &Connection, new: &NewPalette) -> Result<Palette> {
    let id = Uuid::new_v4().to_string();
    let created_at = Utc::now().to_rfc3339();
    let colors_json = serde_json::to_string(&new.colors).unwrap_or_else(|_| "[]".into());
    let tags_json = serde_json::to_string(&new.tags).unwrap_or_else(|_| "[]".into());

    conn.execute(
        "INSERT INTO palettes (id, name, colors, tags, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![id, new.name, colors_json, tags_json, created_at],
    )?;

    Ok(Palette {
        id,
        name: new.name.clone(),
        colors: new.colors.clone(),
        tags: new.tags.clone(),
        created_at,
    })
}

/// Retrieve a palette by ID
pub fn get_palette(conn: &Connection, id: &str) -> Result<Palette> {
    let mut stmt =
        conn.prepare("SELECT id, name, colors, tags, created_at FROM palettes WHERE id = ?1")?;

    let palette = stmt.query_row(params![id], |row| {
        let colors_raw: String = row.get(2)?;
        let tags_raw: String = row.get(3)?;

        let colors: Vec<String> = serde_json::from_str(&colors_raw).unwrap_or_default();
        let tags: Vec<String> = serde_json::from_str(&tags_raw).unwrap_or_default();

        Ok(Palette {
            id: row.get(0)?,
            name: row.get(1)?,
            colors,
            tags,
            created_at: row.get(4)?,
        })
    })?;

    Ok(palette)
}

/// List all available palettes
pub fn list_palettes(conn: &Connection) -> Result<Vec<Palette>> {
    let mut stmt = conn.prepare(
        "SELECT id, name, colors, tags, created_at FROM palettes ORDER BY created_at DESC",
    )?;

    let rows = stmt.query_map([], |row| {
        let colors_raw: String = row.get(2)?;
        let tags_raw: String = row.get(3)?;

        let colors: Vec<String> = serde_json::from_str(&colors_raw).unwrap_or_default();
        let tags: Vec<String> = serde_json::from_str(&tags_raw).unwrap_or_default();

        Ok(Palette {
            id: row.get(0)?,
            name: row.get(1)?,
            colors,
            tags,
            created_at: row.get(4)?,
        })
    })?;

    let mut list = Vec::new();
    for r in rows {
        list.push(r?);
    }
    Ok(list)
}

/// Delete a palette by ID
pub fn delete_palette(conn: &Connection, id: &str) -> Result<()> {
    let affected = conn.execute("DELETE FROM palettes WHERE id = ?1", params![id])?;
    if affected == 0 {
        return Err(crate::error::OpenBlocksError::PaletteNotFound(format!(
            "Palette ID {} not found",
            id
        )));
    }
    Ok(())
}

/// Seed the database with popular color palettes inspired by Color Hunt
pub fn seed_popular_palettes(conn: &Connection) -> Result<usize> {
    // Check if we already have palettes seeded
    let count: i64 = conn.query_row("SELECT COUNT(*) FROM palettes", [], |row| row.get(0))?;
    if count > 0 {
        return Ok(0); // Already seeded
    }

    let default_palettes = vec![
        NewPalette {
            name: "Nordic Frost".into(),
            colors: vec![
                "#2e3440".into(),
                "#3b4252".into(),
                "#88c0d0".into(),
                "#eceff4".into(),
            ],
            tags: vec![
                "nordic".into(),
                "cold".into(),
                "clean".into(),
                "pastel".into(),
            ],
        },
        NewPalette {
            name: "Sunset Glow".into(),
            colors: vec![
                "#f05454".into(),
                "#30475e".into(),
                "#1f4068".into(),
                "#e8e8e8".into(),
            ],
            tags: vec![
                "warm".into(),
                "sunset".into(),
                "retro".into(),
                "contrast".into(),
            ],
        },
        NewPalette {
            name: "Vintage Cardboard".into(),
            colors: vec![
                "#f7f7f7".into(),
                "#efe1d1".into(),
                "#a78a7f".into(),
                "#393e46".into(),
            ],
            tags: vec![
                "vintage".into(),
                "retro".into(),
                "warm".into(),
                "paper".into(),
            ],
        },
        NewPalette {
            name: "Cyberpunk Neon".into(),
            colors: vec![
                "#0f0c1b".into(),
                "#1f1a3a".into(),
                "#ff007f".into(),
                "#00f0ff".into(),
            ],
            tags: vec![
                "cyberpunk".into(),
                "neon".into(),
                "dark".into(),
                "futuristic".into(),
            ],
        },
        NewPalette {
            name: "Midnight Coffee".into(),
            colors: vec![
                "#1b1a17".into(),
                "#a35709".into(),
                "#ff8303".into(),
                "#f0e3ca".into(),
            ],
            tags: vec!["coffee".into(), "dark".into(), "warm".into(), "cozy".into()],
        },
        NewPalette {
            name: "Mint Fresh".into(),
            colors: vec![
                "#1e2022".into(),
                "#52616b".into(),
                "#c9d6df".into(),
                "#f0f5f9".into(),
            ],
            tags: vec![
                "mint".into(),
                "fresh".into(),
                "corporate".into(),
                "clean".into(),
            ],
        },
        NewPalette {
            name: "Lavender Sunset".into(),
            colors: vec![
                "#3d2b56".into(),
                "#5e4585".into(),
                "#8b6cb8".into(),
                "#ffbfa3".into(),
            ],
            tags: vec![
                "lavender".into(),
                "pastel".into(),
                "sunset".into(),
                "purple".into(),
            ],
        },
        NewPalette {
            name: "Desert Cactus".into(),
            colors: vec![
                "#2c3531".into(),
                "#116466".into(),
                "#d9b08c".into(),
                "#ffcb9a".into(),
            ],
            tags: vec![
                "desert".into(),
                "cactus".into(),
                "natural".into(),
                "earthy".into(),
            ],
        },
        NewPalette {
            name: "Sleek Corporate".into(),
            colors: vec![
                "#222831".into(),
                "#393e46".into(),
                "#00adb5".into(),
                "#eeeeee".into(),
            ],
            tags: vec![
                "corporate".into(),
                "sleek".into(),
                "modern".into(),
                "tech".into(),
            ],
        },
        NewPalette {
            name: "Golden Hour".into(),
            colors: vec![
                "#2b2e4a".into(),
                "#e84545".into(),
                "#903749".into(),
                "#53354a".into(),
            ],
            tags: vec![
                "sunset".into(),
                "gold".into(),
                "warm".into(),
                "royal".into(),
                "autumn".into(),
            ],
        },
        NewPalette {
            name: "Bauhaus Modernism".into(),
            colors: vec![
                "#1a1a1a".into(),
                "#e63946".into(),
                "#ffb703".into(),
                "#1d3557".into(),
            ],
            tags: vec![
                "bauhaus".into(),
                "primary".into(),
                "design".into(),
                "classic".into(),
            ],
        },
        NewPalette {
            name: "Fontshare Editorial".into(),
            colors: vec![
                "#0f0f11".into(),
                "#f5f5f7".into(),
                "#3a3a3c".into(),
                "#d1d1d6".into(),
            ],
            tags: vec![
                "typography".into(),
                "editorial".into(),
                "minimalist".into(),
                "gray".into(),
            ],
        },
        NewPalette {
            name: "LiveKit Aura".into(),
            colors: vec![
                "#080710".into(),
                "#5f26cd".into(),
                "#12a0ff".into(),
                "#ffffff".into(),
            ],
            tags: vec![
                "glow".into(),
                "aura".into(),
                "livekit".into(),
                "purple".into(),
            ],
        },
        NewPalette {
            name: "Aceternity Dark".into(),
            colors: vec![
                "#000000".into(),
                "#09090b".into(),
                "#3b82f6".into(),
                "#8b5cf6".into(),
            ],
            tags: vec![
                "aceternity".into(),
                "sleek".into(),
                "dark".into(),
                "blue".into(),
            ],
        },
        NewPalette {
            name: "Muted Autumn".into(),
            colors: vec![
                "#2c302e".into(),
                "#4f5d75".into(),
                "#d1ac00".into(),
                "#f7ebec".into(),
            ],
            tags: vec![
                "autumn".into(),
                "muted".into(),
                "cozy".into(),
                "natural".into(),
            ],
        },
    ];

    let mut seeded = 0;
    for pal in default_palettes {
        insert_palette(conn, &pal)?;
        seeded += 1;
    }

    Ok(seeded)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::connection::open_connection;
    use crate::db::migrations::get_migrations;

    fn setup_test_db() -> Connection {
        let mut conn = open_connection(":memory:").unwrap();
        let migrations = get_migrations();
        migrations.to_latest(&mut conn).unwrap();
        conn
    }

    #[test]
    fn test_insert_and_get_palette() {
        let conn = setup_test_db();
        let new_palette = NewPalette {
            name: "Test Forest".into(),
            colors: vec![
                "#1b4332".into(),
                "#2d6a4f".into(),
                "#40916c".into(),
                "#52b788".into(),
            ],
            tags: vec!["green".into(), "nature".into()],
        };

        let inserted = insert_palette(&conn, &new_palette).unwrap();
        assert_eq!(inserted.name, "Test Forest");
        assert_eq!(inserted.colors.len(), 4);

        let fetched = get_palette(&conn, &inserted.id).unwrap();
        assert_eq!(fetched.id, inserted.id);
        assert_eq!(fetched.name, "Test Forest");
    }

    #[test]
    fn test_delete_palette() {
        let conn = setup_test_db();
        let new_palette = NewPalette {
            name: "Delete Me".into(),
            colors: vec![
                "#000000".into(),
                "#111111".into(),
                "#222222".into(),
                "#333333".into(),
            ],
            tags: vec![],
        };

        let inserted = insert_palette(&conn, &new_palette).unwrap();
        assert!(delete_palette(&conn, &inserted.id).is_ok());
        assert!(get_palette(&conn, &inserted.id).is_err());
    }

    #[test]
    fn test_seed_popular_palettes() {
        let conn = setup_test_db();
        let seeded = seed_popular_palettes(&conn).unwrap();
        assert_eq!(seeded, 15);

        let list = list_palettes(&conn).unwrap();
        assert_eq!(list.len(), 15);

        // Seeding again should be a no-op
        let seeded_again = seed_popular_palettes(&conn).unwrap();
        assert_eq!(seeded_again, 0);
    }
}

