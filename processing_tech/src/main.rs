//! YC Data Processor - sequential pipeline.
//!
//! Runs 8 passes in order:
//!   1. slug, name, url (from URL and header)
//!   2. batch (season + year)
//!   3. status + location
//!   4. tags (ALL_CAPS, filtered by location from pass3)
//!   5. details (tagline, sidebar fields, is_hiring)
//!   6. founders
//!   7. news
//!   8. links (with founder matching from pass6)

mod db;
mod pass1;
mod pass2;
mod pass3;
mod pass4;
mod pass5;
mod pass6;
mod pass7;
mod pass8;
mod utils;

use anyhow::Result;

fn main() -> Result<()> {
    println!("YC Data Processor");
    println!("=================\n");

    let conn = db::connect()?;
    println!("Database: {:?}\n", db::path());

    db::create_tables(&conn)?;
    println!("Tables ready.\n");

    println!("Loading pages...");
    let pages = db::fetch_pages(&conn)?;
    println!("Loaded {} pages.\n", pages.len());

    if pages.is_empty() {
        println!("No pages found.");
        return Ok(());
    }

    // Run all passes in a single transaction for speed
    conn.execute("BEGIN TRANSACTION", [])?;

    // Pass 1: slug, name, url
    println!("Pass 1: slug, name, url");
    let n = pass1::run(&conn, &pages)?;
    println!("  -> {} companies\n", n);

    // Pass 2: batch (season + year)
    println!("Pass 2: batch");
    let n = pass2::run(&conn, &pages)?;
    println!("  -> {} updated\n", n);

    // Pass 3: status + location
    println!("Pass 3: status + location");
    let n = pass3::run(&conn, &pages)?;
    println!("  -> {} updated\n", n);

    // Pass 4: tags (uses location from pass3 to filter)
    println!("Pass 4: tags");
    let n = pass4::run(&conn, &pages)?;
    println!("  -> {} tags\n", n);

    // Pass 5: details (tagline, sidebar fields)
    println!("Pass 5: details");
    let n = pass5::run(&conn, &pages)?;
    println!("  -> {} updated\n", n);

    // Pass 6: founders
    println!("Pass 6: founders");
    let n = pass6::run(&conn, &pages)?;
    println!("  -> {} founders\n", n);

    // Pass 7: news
    println!("Pass 7: news");
    let n = pass7::run(&conn, &pages)?;
    println!("  -> {} news items\n", n);

    // Pass 8: links (uses founders from pass6)
    println!("Pass 8: links");
    let n = pass8::run(&conn, &pages)?;
    println!("  -> {} links\n", n);

    conn.execute("COMMIT", [])?;

    pass8::print_stats(&conn)?;
    db::stats(&conn)?;

    println!("\nDone.");
    Ok(())
}
