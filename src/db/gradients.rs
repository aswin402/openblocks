use rusqlite::{params, Connection};
use crate::error::Result;
use crate::models::gradient::{Gradient, NewGradient};
use uuid::Uuid;
use chrono::Utc;

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
    let mut stmt = conn.prepare(
        "SELECT id, name, css, colors, tags, created_at FROM gradients WHERE id = ?1"
    )?;
    
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
        "SELECT id, name, css, colors, tags, created_at FROM gradients ORDER BY created_at DESC"
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
        return Err(crate::error::OpenBlocksError::PaletteNotFound(format!("Gradient ID {} not found", id)));
    }
    Ok(())
}

/// Seed the database with popular gradients inspired by Coolors Gradients
pub fn seed_popular_gradients(conn: &Connection) -> Result<usize> {
    // Check if we already have gradients seeded
    let count: i64 = conn.query_row("SELECT COUNT(*) FROM gradients", [], |row| row.get(0))?;
    if count > 0 {
        return Ok(0); // Already seeded
    }

    let default_gradients = vec![
        NewGradient {
            name: "Warm Flame".into(),
            css: "linear-gradient(135deg, #ff9a9e 0%, #fecfef 99%, #fecfef 100%)".into(),
            colors: vec!["#ff9a9e".into(), "#fecfef".into()],
            tags: vec!["warm".into(), "pink".into(), "romantic".into(), "sunset".into()],
        },
        NewGradient {
            name: "Ocean Breeze".into(),
            css: "linear-gradient(135deg, #84ffc9 0%, #aab2ff 48%, #eca0ff 100%)".into(),
            colors: vec!["#84ffc9".into(), "#aab2ff".into(), "#eca0ff".into()],
            tags: vec!["ocean".into(), "breeze".into(), "fresh".into(), "gradient".into()],
        },
        NewGradient {
            name: "Deep Blue".into(),
            css: "linear-gradient(135deg, #0652dd 0%, #12cbc4 100%)".into(),
            colors: vec!["#0652dd".into(), "#12cbc4".into()],
            tags: vec!["blue".into(), "ocean".into(), "corporate".into(), "tech".into()],
        },
        NewGradient {
            name: "Neon Glow".into(),
            css: "linear-gradient(135deg, #ff007f 0%, #7f00ff 100%)".into(),
            colors: vec!["#ff007f".into(), "#7f00ff".into()],
            tags: vec!["neon".into(), "purple".into(), "cyberpunk".into(), "vibrant".into()],
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
            tags: vec!["mint".into(), "fresh".into(), "clean".into(), "nature".into()],
        },
        NewGradient {
            name: "Purple Paradise".into(),
            css: "linear-gradient(135deg, #667eea 0%, #764ba2 100%)".into(),
            colors: vec!["#667eea".into(), "#764ba2".into()],
            tags: vec!["purple".into(), "paradise".into(), "lavender".into(), "sleek".into()],
        },
        NewGradient {
            name: "Citrus Blend".into(),
            css: "linear-gradient(135deg, #f39c12 0%, #d35400 100%)".into(),
            colors: vec!["#f39c12".into(), "#d35400".into()],
            tags: vec!["orange".into(), "citrus".into(), "vibrant".into(), "warm".into()],
        },
        NewGradient {
            name: "Cotton Candy".into(),
            css: "linear-gradient(135deg, #ff9a9e 0%, #a1c4fd 100%)".into(),
            colors: vec!["#ff9a9e".into(), "#a1c4fd".into()],
            tags: vec!["pastel".into(), "sweet".into(), "cotton".into(), "candy".into()],
        },
        NewGradient {
            name: "Slate Gray".into(),
            css: "linear-gradient(135deg, #708090 0%, #2f4f4f 100%)".into(),
            colors: vec!["#708090".into(), "#2f4f4f".into()],
            tags: vec!["dark".into(), "slate".into(), "corporate".into(), "minimal".into()],
        }
    ];

    let mut seeded = 0;
    for grad in default_gradients {
        insert_gradient(conn, &grad)?;
        seeded += 1;
    }
    
    Ok(seeded)
}
