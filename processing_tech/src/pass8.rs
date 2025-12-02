//! Pass 8: Extract and classify external links.
//!
//! Links come from external_links JSON field.
//! - Skip internal YC links
//! - Classify by URL pattern (linkedin.com/in, twitter.com, etc.)
//! - Match founder links to founders by name in URL
//!
//! Inserts into links table.

use anyhow::Result;
use rusqlite::Connection;

use crate::{db, utils};

const INTERNAL: &[&str] = &["ycombinator.com", "startupschool.org"];

pub fn run(conn: &Connection, pages: &[(String, Option<String>, Option<String>)]) -> Result<usize> {
    let mut count = 0;

    for (url, _, links_json) in pages {
        let slug = match utils::slug_from_url(url) {
            Some(s) => s,
            None => continue,
        };

        let links_json = match links_json {
            Some(j) => j,
            None => continue,
        };

        let urls: Vec<String> = match serde_json::from_str(links_json) {
            Ok(u) => u,
            Err(_) => continue,
        };

        // Get founders for this company
        let founders = get_founders(conn, slug).unwrap_or_default();

        for link_url in urls {
            // Skip internal
            if INTERNAL.iter().any(|d| link_url.contains(d)) {
                continue;
            }

            let pattern = url_pattern(&link_url);
            let founder_id = match_founder(&link_url, &founders);

            db::insert(
                conn,
                "links",
                &[
                    ("company_slug", &slug as &dyn rusqlite::ToSql),
                    ("founder_id", &founder_id),
                    ("url", &link_url),
                    ("pattern", &pattern),
                ],
            )?;
            count += 1;
        }
    }

    Ok(count)
}


fn url_pattern(url: &str) -> Option<String> {
    let url = url
        .strip_prefix("https://")
        .or_else(|| url.strip_prefix("http://"))?;

    let domain = url.split('/').next()?;
    let domain = domain.strip_prefix("www.").unwrap_or(domain);

    let parts: Vec<&str> = url.splitn(3, '/').collect();
    match parts.get(1) {
        Some(path) if !path.is_empty() => Some(format!("{}/{}", domain, path)),
        _ => Some(domain.to_string()),
    }
}

fn get_founders(conn: &Connection, company_slug: &str) -> Result<Vec<(i64, String)>> {
    let mut stmt = conn.prepare("SELECT id, name FROM founders WHERE company_slug = ?")?;
    let rows = stmt
        .query_map([company_slug], |row| Ok((row.get(0)?, row.get(1)?)))?
        .filter_map(|r| r.ok())
        .collect();
    Ok(rows)
}

fn match_founder(url: &str, founders: &[(i64, String)]) -> Option<i64> {
    let url_lower = url.to_lowercase();

    // Only try to match personal profile links
    if !url_lower.contains("linkedin.com/in/")
        && !url_lower.contains("twitter.com/")
        && !url_lower.contains("x.com/")
    {
        return None;
    }

    for (id, name) in founders {
        let name_lower = name.to_lowercase();
        let name_parts: Vec<&str> = name_lower.split_whitespace().collect();

        let variants = [
            name_parts.join("-"),
            name_parts.join(""),
            name_parts.join("_"),
        ];

        // Try last name only
        if let Some(last) = name_parts.last() {
            if last.len() >= 4 && url_lower.contains(last) {
                return Some(*id);
            }
        }

        for variant in &variants {
            if !variant.is_empty() && variant.len() >= 4 && url_lower.contains(variant) {
                return Some(*id);
            }
        }
    }

    None
}

pub fn print_stats(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare(
        "SELECT pattern, COUNT(*) as cnt FROM links
         WHERE pattern IS NOT NULL
         GROUP BY pattern ORDER BY cnt DESC LIMIT 15",
    )?;

    println!("\nTop 15 link patterns:");
    let rows = stmt.query_map([], |row| {
        Ok((row.get::<_, Option<String>>(0)?, row.get::<_, i64>(1)?))
    })?;

    for row in rows.flatten() {
        let (pattern, cnt) = row;
        println!("  {:40} {}", pattern.unwrap_or_default(), cnt);
    }

    let founder_links: i64 = conn.query_row(
        "SELECT COUNT(*) FROM links WHERE founder_id IS NOT NULL",
        [],
        |r| r.get(0),
    )?;
    let company_links: i64 = conn.query_row(
        "SELECT COUNT(*) FROM links WHERE founder_id IS NULL",
        [],
        |r| r.get(0),
    )?;

    println!("\nLink breakdown:");
    println!("  Company links: {}", company_links);
    println!("  Founder links: {}", founder_links);

    Ok(())
}
