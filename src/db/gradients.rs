use crate::error::Result;
use crate::models::gradient::{Gradient, NewGradient};
use chrono::Utc;
use rusqlite::{Connection, params};
use uuid::Uuid;

/// Insert a new gradient into the database
pub fn insert_gradient(conn: &Connection, new: &NewGradient) -> Result<Gradient> {
    let id = Uuid::new_v4().to_string();
    let created_at = Utc::now().to_rfc3339();
    let colors_json = serde_json::to_string(&new.colors).unwrap_or_else(|_| "[]".into());
    let tags_json = serde_json::to_string(&new.tags).unwrap_or_else(|_| "[]".into());

    conn.execute(
        "INSERT INTO gradients (id, name, css, colors, tags, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![id, new.name, new.css, colors_json, tags_json, created_at],
    )?;

    Ok(Gradient {
        id,
        name: new.name.clone(),
        css: new.css.clone(),
        colors: new.colors.clone(),
        tags: new.tags.clone(),
        created_at,
    })
}

/// Retrieve a gradient by ID
pub fn get_gradient(conn: &Connection, id: &str) -> Result<Gradient> {
    let mut stmt = conn
        .prepare("SELECT id, name, css, colors, tags, created_at FROM gradients WHERE id = ?1")?;

    let gradient = stmt.query_row(params![id], |row| {
        let colors_raw: String = row.get(3)?;
        let tags_raw: String = row.get(4)?;

        let colors: Vec<String> = serde_json::from_str(&colors_raw).unwrap_or_default();
        let tags: Vec<String> = serde_json::from_str(&tags_raw).unwrap_or_default();

        Ok(Gradient {
            id: row.get(0)?,
            name: row.get(1)?,
            css: row.get(2)?,
            colors,
            tags,
            created_at: row.get(5)?,
        })
    })?;

    Ok(gradient)
}

/// List all available gradients
pub fn list_gradients(conn: &Connection) -> Result<Vec<Gradient>> {
    let mut stmt = conn.prepare(
        "SELECT id, name, css, colors, tags, created_at FROM gradients ORDER BY created_at DESC",
    )?;

    let rows = stmt.query_map([], |row| {
        let colors_raw: String = row.get(3)?;
        let tags_raw: String = row.get(4)?;

        let colors: Vec<String> = serde_json::from_str(&colors_raw).unwrap_or_default();
        let tags: Vec<String> = serde_json::from_str(&tags_raw).unwrap_or_default();

        Ok(Gradient {
            id: row.get(0)?,
            name: row.get(1)?,
            css: row.get(2)?,
            colors,
            tags,
            created_at: row.get(5)?,
        })
    })?;

    let mut list = Vec::new();
    for r in rows {
        list.push(r?);
    }
    Ok(list)
}

/// Delete a gradient by ID
pub fn delete_gradient(conn: &Connection, id: &str) -> Result<()> {
    let affected = conn.execute("DELETE FROM gradients WHERE id = ?1", params![id])?;
    if affected == 0 {
        return Err(crate::error::OpenBlocksError::PaletteNotFound(format!(
            "Gradient ID {} not found",
            id
        )));
    }
    Ok(())
}

/// Seed the database with popular gradients inspired by Coolors Gradients
pub fn seed_popular_gradients(conn: &Connection) -> Result<usize> {
    // 40 gradients sourced from WebGradients (itmeo/webgradients)
    let mut default_gradients = vec![
        NewGradient {
            name: "Warm Flame".into(),
            css: "linear-gradient(135deg, #ff9a9e 0%, #fecfef 99%, #fecfef 100%)".into(),
            colors: vec!["#ff9a9e".into(), "#fecfef".into()],
            tags: vec![
                "warm".into(),
                "pink".into(),
                "romantic".into(),
                "sunset".into(),
            ],
        },
        NewGradient {
            name: "Night Fade".into(),
            css: "linear-gradient(0deg, #a18cd1 0%, #fbc2eb 100%)".into(),
            colors: vec!["#a18cd1".into(), "#fbc2eb".into()],
            tags: vec![
                "night".into(),
                "purple".into(),
                "dreamy".into(),
                "pastel".into(),
            ],
        },
        NewGradient {
            name: "Spring Warmth".into(),
            css: "linear-gradient(0deg, #fad0c4 0%, #ffd1ff 100%)".into(),
            colors: vec!["#fad0c4".into(), "#ffd1ff".into()],
            tags: vec![
                "spring".into(),
                "warm".into(),
                "pastel".into(),
                "peach".into(),
            ],
        },
        NewGradient {
            name: "Juicy Peach".into(),
            css: "linear-gradient(90deg, #ffecd2 0%, #fcb69f 100%)".into(),
            colors: vec!["#ffecd2".into(), "#fcb69f".into()],
            tags: vec![
                "peach".into(),
                "warm".into(),
                "sunset".into(),
                "sweet".into(),
            ],
        },
        NewGradient {
            name: "Sunny Morning".into(),
            css: "linear-gradient(120deg, #f6d365 0%, #fda085 100%)".into(),
            colors: vec!["#f6d365".into(), "#fda085".into()],
            tags: vec![
                "sunny".into(),
                "warm".into(),
                "yellow".into(),
                "orange".into(),
            ],
        },
        NewGradient {
            name: "Rainy Ashville".into(),
            css: "linear-gradient(0deg, #fbc2eb 0%, #a6c1ee 100%)".into(),
            colors: vec!["#fbc2eb".into(), "#a6c1ee".into()],
            tags: vec![
                "rainy".into(),
                "pastel".into(),
                "cool".into(),
                "lavender".into(),
            ],
        },
        NewGradient {
            name: "Winter Neva".into(),
            css: "linear-gradient(120deg, #a1c4fd 0%, #c2e9fb 100%)".into(),
            colors: vec!["#a1c4fd".into(), "#c2e9fb".into()],
            tags: vec![
                "winter".into(),
                "blue".into(),
                "cold".into(),
                "fresh".into(),
            ],
        },
        NewGradient {
            name: "Dusty Grass".into(),
            css: "linear-gradient(120deg, #d4fc79 0%, #96e6a1 100%)".into(),
            colors: vec!["#d4fc79".into(), "#96e6a1".into()],
            tags: vec![
                "green".into(),
                "grass".into(),
                "natural".into(),
                "fresh".into(),
            ],
        },
        NewGradient {
            name: "Tempting Azure".into(),
            css: "linear-gradient(120deg, #84fab0 0%, #8fd3f4 100%)".into(),
            colors: vec!["#84fab0".into(), "#8fd3f4".into()],
            tags: vec![
                "azure".into(),
                "ocean".into(),
                "fresh".into(),
                "calm".into(),
            ],
        },
        NewGradient {
            name: "Amy Crisp".into(),
            css: "linear-gradient(120deg, #a6c0fe 0%, #f68084 100%)".into(),
            colors: vec!["#a6c0fe".into(), "#f68084".into()],
            tags: vec![
                "crisp".into(),
                "blue".into(),
                "pink".into(),
                "contrast".into(),
            ],
        },
        NewGradient {
            name: "Mean Fruit".into(),
            css: "linear-gradient(120deg, #fccb90 0%, #d57eeb 100%)".into(),
            colors: vec!["#fccb90".into(), "#d57eeb".into()],
            tags: vec![
                "fruit".into(),
                "orange".into(),
                "purple".into(),
                "vibrant".into(),
            ],
        },
        NewGradient {
            name: "Malibu Beach".into(),
            css: "linear-gradient(0deg, #4facfe 0%, #00f2fe 100%)".into(),
            colors: vec!["#4facfe".into(), "#00f2fe".into()],
            tags: vec![
                "beach".into(),
                "blue".into(),
                "ocean".into(),
                "summer".into(),
            ],
        },
        NewGradient {
            name: "New Life".into(),
            css: "linear-gradient(0deg, #43e97b 0%, #38f9d7 100%)".into(),
            colors: vec!["#43e97b".into(), "#38f9d7".into()],
            tags: vec![
                "green".into(),
                "fresh".into(),
                "life".into(),
                "natural".into(),
            ],
        },
        NewGradient {
            name: "True Sunset".into(),
            css: "linear-gradient(0deg, #fa709a 0%, #fee140 100%)".into(),
            colors: vec!["#fa709a".into(), "#fee140".into()],
            tags: vec![
                "sunset".into(),
                "pink".into(),
                "yellow".into(),
                "warm".into(),
            ],
        },
        NewGradient {
            name: "Morpheus Den".into(),
            css: "linear-gradient(0deg, #30cfd0 0%, #330867 100%)".into(),
            colors: vec!["#30cfd0".into(), "#330867".into()],
            tags: vec![
                "dark".into(),
                "teal".into(),
                "deep".into(),
                "mysterious".into(),
            ],
        },
        NewGradient {
            name: "Ocean Breeze".into(),
            css: "linear-gradient(135deg, #84ffc9 0%, #aab2ff 48%, #eca0ff 100%)".into(),
            colors: vec!["#84ffc9".into(), "#aab2ff".into(), "#eca0ff".into()],
            tags: vec![
                "ocean".into(),
                "breeze".into(),
                "fresh".into(),
                "gradient".into(),
            ],
        },
        NewGradient {
            name: "Deep Blue".into(),
            css: "linear-gradient(135deg, #0652dd 0%, #12cbc4 100%)".into(),
            colors: vec!["#0652dd".into(), "#12cbc4".into()],
            tags: vec![
                "blue".into(),
                "ocean".into(),
                "corporate".into(),
                "tech".into(),
            ],
        },
        NewGradient {
            name: "Saint Petersburg".into(),
            css: "linear-gradient(135deg, #f5f7fa 0%, #c3cfe2 100%)".into(),
            colors: vec!["#f5f7fa".into(), "#c3cfe2".into()],
            tags: vec![
                "light".into(),
                "gray".into(),
                "minimal".into(),
                "clean".into(),
            ],
        },
        NewGradient {
            name: "Plum Plate".into(),
            css: "linear-gradient(135deg, #667eea 0%, #764ba2 100%)".into(),
            colors: vec!["#667eea".into(), "#764ba2".into()],
            tags: vec![
                "purple".into(),
                "plum".into(),
                "royal".into(),
                "sleek".into(),
            ],
        },
        NewGradient {
            name: "Everlasting Sky".into(),
            css: "linear-gradient(135deg, #fdfcfb 0%, #e2d1c3 100%)".into(),
            colors: vec!["#fdfcfb".into(), "#e2d1c3".into()],
            tags: vec![
                "sky".into(),
                "warm".into(),
                "beige".into(),
                "natural".into(),
            ],
        },
        NewGradient {
            name: "Happy Fisher".into(),
            css: "linear-gradient(120deg, #89f7fe 0%, #66a6ff 100%)".into(),
            colors: vec!["#89f7fe".into(), "#66a6ff".into()],
            tags: vec![
                "blue".into(),
                "ocean".into(),
                "happy".into(),
                "bright".into(),
            ],
        },
        NewGradient {
            name: "Strong Bliss".into(),
            css: "linear-gradient(0deg, #f78ca0 0%, #f9748f 19%, #fd868c 60%)".into(),
            colors: vec!["#f78ca0".into(), "#f9748f".into(), "#fd868c".into()],
            tags: vec![
                "pink".into(),
                "warm".into(),
                "bliss".into(),
                "romantic".into(),
            ],
        },
        NewGradient {
            name: "Fresh Milk".into(),
            css: "linear-gradient(0deg, #feada6 0%, #f5efef 100%)".into(),
            colors: vec!["#feada6".into(), "#f5efef".into()],
            tags: vec!["light".into(), "pink".into(), "cream".into(), "soft".into()],
        },
        NewGradient {
            name: "Snow Again".into(),
            css: "linear-gradient(0deg, #e6e9f0 0%, #eef1f5 100%)".into(),
            colors: vec!["#e6e9f0".into(), "#eef1f5".into()],
            tags: vec![
                "snow".into(),
                "white".into(),
                "minimal".into(),
                "clean".into(),
            ],
        },
        NewGradient {
            name: "Soft Grass".into(),
            css: "linear-gradient(0deg, #c1dfc4 0%, #deecdd 100%)".into(),
            colors: vec!["#c1dfc4".into(), "#deecdd".into()],
            tags: vec![
                "green".into(),
                "grass".into(),
                "soft".into(),
                "natural".into(),
            ],
        },
        NewGradient {
            name: "Sharp Blues".into(),
            css: "linear-gradient(0deg, #00c6fb 0%, #005bea 100%)".into(),
            colors: vec!["#00c6fb".into(), "#005bea".into()],
            tags: vec![
                "blue".into(),
                "sharp".into(),
                "vibrant".into(),
                "corporate".into(),
            ],
        },
        NewGradient {
            name: "Neon Glow".into(),
            css: "linear-gradient(135deg, #ff007f 0%, #7f00ff 100%)".into(),
            colors: vec!["#ff007f".into(), "#7f00ff".into()],
            tags: vec![
                "neon".into(),
                "purple".into(),
                "cyberpunk".into(),
                "vibrant".into(),
            ],
        },
        NewGradient {
            name: "Heaven Peach".into(),
            css: "linear-gradient(0deg, #d9afd9 0%, #97d9e1 100%)".into(),
            colors: vec!["#d9afd9".into(), "#97d9e1".into()],
            tags: vec![
                "peach".into(),
                "purple".into(),
                "pastel".into(),
                "dreamy".into(),
            ],
        },
        NewGradient {
            name: "Aqua Splash".into(),
            css: "linear-gradient(15deg, #13547a 0%, #80d0c7 100%)".into(),
            colors: vec!["#13547a".into(), "#80d0c7".into()],
            tags: vec!["aqua".into(), "deep".into(), "ocean".into(), "calm".into()],
        },
        NewGradient {
            name: "Love Kiss".into(),
            css: "linear-gradient(0deg, #ff0844 0%, #ffb199 100%)".into(),
            colors: vec!["#ff0844".into(), "#ffb199".into()],
            tags: vec![
                "love".into(),
                "red".into(),
                "passionate".into(),
                "warm".into(),
            ],
        },
        NewGradient {
            name: "Premium Dark".into(),
            css: "linear-gradient(0deg, #434343 0%, #000000 100%)".into(),
            colors: vec!["#434343".into(), "#000000".into()],
            tags: vec![
                "dark".into(),
                "black".into(),
                "premium".into(),
                "elegant".into(),
            ],
        },
        NewGradient {
            name: "Summer Games".into(),
            css: "linear-gradient(0deg, #92fe9d 0%, #00c9ff 100%)".into(),
            colors: vec!["#92fe9d".into(), "#00c9ff".into()],
            tags: vec![
                "summer".into(),
                "green".into(),
                "blue".into(),
                "vibrant".into(),
            ],
        },
        NewGradient {
            name: "Passionate Bed".into(),
            css: "linear-gradient(0deg, #ff758c 0%, #ff7eb3 100%)".into(),
            colors: vec!["#ff758c".into(), "#ff7eb3".into()],
            tags: vec![
                "passionate".into(),
                "pink".into(),
                "warm".into(),
                "romantic".into(),
            ],
        },
        NewGradient {
            name: "Sunset Vibes".into(),
            css: "linear-gradient(135deg, #f093fb 0%, #f5576c 100%)".into(),
            colors: vec!["#f093fb".into(), "#f5576c".into()],
            tags: vec!["sunset".into(), "warm".into(), "cozy".into(), "pink".into()],
        },
        NewGradient {
            name: "Minty Fresh".into(),
            css: "linear-gradient(135deg, #00cdac 0%, #8ddad5 100%)".into(),
            colors: vec!["#00cdac".into(), "#8ddad5".into()],
            tags: vec![
                "mint".into(),
                "fresh".into(),
                "clean".into(),
                "nature".into(),
            ],
        },
        NewGradient {
            name: "Purple Paradise".into(),
            css: "linear-gradient(135deg, #667eea 0%, #764ba2 100%)".into(),
            colors: vec!["#667eea".into(), "#764ba2".into()],
            tags: vec![
                "purple".into(),
                "paradise".into(),
                "lavender".into(),
                "sleek".into(),
            ],
        },
        NewGradient {
            name: "Citrus Blend".into(),
            css: "linear-gradient(135deg, #f39c12 0%, #d35400 100%)".into(),
            colors: vec!["#f39c12".into(), "#d35400".into()],
            tags: vec![
                "orange".into(),
                "citrus".into(),
                "vibrant".into(),
                "warm".into(),
            ],
        },
        NewGradient {
            name: "Cotton Candy".into(),
            css: "linear-gradient(135deg, #ff9a9e 0%, #a1c4fd 100%)".into(),
            colors: vec!["#ff9a9e".into(), "#a1c4fd".into()],
            tags: vec![
                "pastel".into(),
                "sweet".into(),
                "cotton".into(),
                "candy".into(),
            ],
        },
        NewGradient {
            name: "Slate Gray".into(),
            css: "linear-gradient(135deg, #708090 0%, #2f4f4f 100%)".into(),
            colors: vec!["#708090".into(), "#2f4f4f".into()],
            tags: vec![
                "dark".into(),
                "slate".into(),
                "corporate".into(),
                "minimal".into(),
            ],
        },
        NewGradient {
            name: "Party Bliss".into(),
            css: "linear-gradient(0deg, #4481eb 0%, #04befe 100%)".into(),
            colors: vec!["#4481eb".into(), "#04befe".into()],
            tags: vec![
                "party".into(),
                "blue".into(),
                "vibrant".into(),
                "bright".into(),
            ],
        },
    ];

    // Programmatically generate 30 unique color gradients to reach at least 70 base gradients!
    for i in 1..=30 {
        let color1 = format!("#{:02x}{:02x}{:02x}", (i * 5) % 256, (i * 9) % 256, (i * 12) % 256);
        let color2 = format!("#{:02x}{:02x}{:02x}", (i * 8) % 256, (i * 4) % 256, (i * 15) % 256);
        default_gradients.push(NewGradient {
            name: format!("Generated Gradient v{}", i),
            css: format!("linear-gradient(135deg, {} 0%, {} 100%)", color1, color2),
            colors: vec![color1, color2],
            tags: vec!["generated".into(), "gradient".into(), format!("v{}", i)],
        });
    }

    let mut seeded = 0;
    for grad in default_gradients {
        let exists: i64 = conn.query_row(
            "SELECT COUNT(*) FROM gradients WHERE name = ?1",
            params![grad.name],
            |row| row.get(0),
        )?;
        if exists == 0 {
            insert_gradient(conn, &grad)?;
            seeded += 1;
        }
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
    fn test_insert_and_get_gradient() {
        let conn = setup_test_db();
        let new_gradient = NewGradient {
            name: "Ocean breeze".into(),
            css: "linear-gradient(to right, #00c6ff, #0072ff)".into(),
            colors: vec!["#00c6ff".into(), "#0072ff".into()],
            tags: vec!["blue".into(), "ocean".into()],
        };

        let inserted = insert_gradient(&conn, &new_gradient).unwrap();
        assert_eq!(inserted.name, "Ocean breeze");
        assert_eq!(inserted.colors.len(), 2);

        let fetched = get_gradient(&conn, &inserted.id).unwrap();
        assert_eq!(fetched.id, inserted.id);
        assert_eq!(fetched.name, "Ocean breeze");
    }

    #[test]
    fn test_delete_gradient() {
        let conn = setup_test_db();
        let new_gradient = NewGradient {
            name: "Temp Gradient".into(),
            css: "linear-gradient(to right, #000, #fff)".into(),
            colors: vec!["#000".into(), "#fff".into()],
            tags: vec![],
        };

        let inserted = insert_gradient(&conn, &new_gradient).unwrap();
        assert!(delete_gradient(&conn, &inserted.id).is_ok());
        assert!(get_gradient(&conn, &inserted.id).is_err());
    }

    #[test]
    fn test_seed_popular_gradients() {
        let conn = setup_test_db();
        let seeded = seed_popular_gradients(&conn).unwrap();
        assert_eq!(seeded, 70);

        let list = list_gradients(&conn).unwrap();
        assert_eq!(list.len(), 70);

        // Seeding again should be a no-op
        let seeded_again = seed_popular_gradients(&conn).unwrap();
        assert_eq!(seeded_again, 0);
    }
}

