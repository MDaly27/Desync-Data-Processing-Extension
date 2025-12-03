# Desync Data Processing Extension

Compact map of the repo:

- `Python_Scraping/` – Python scrapers and job-runner utilities (YC site + job listings).
- `XML_Sitemaps/` – sitemap fetch/parse loader to seed URLs into SQLite.
- `Sqlite_Database/` – SQLite schema helpers and the working database (`data/yc.sqlite`).
- `pipeline/` – Python orchestrator that ties scraping to Rust post-processing.
- `Rust_Processing/` – Rust binary that parses scraped pages into structured tables.

Quick start:
- Set `DESYNC_API_KEY` in your environment for scraping.
- Ensure the DB exists (run `python -m Sqlite_Database.schema` to create base tables).
- Load URLs: `python XML_Sitemaps/sitemap_parser.py` (or `--local` for cached XMLs).
- Scrape companies: `python pipeline/integrated_pipeline.py` (reads from SQLite, writes back, then calls Rust).
- Scrape job pages: `python Python_Scraping/job_listings/scrape_jobs.py --init --pipeline`.

Data flow:
Sitemaps -> `websites_from_sitemap` -> Python scrapers -> `pagedataobjects` / `jobs_page_data` -> Rust processor -> processed tables.
