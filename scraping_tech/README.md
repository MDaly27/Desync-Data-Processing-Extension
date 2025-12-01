# scraping_tech

Y Combinator website scraper using the Desync AI bulk search API.

```
scraping_tech/
├── data/
│   ├── companies_sitemap.xml   # Cached sitemap with YC company URLs
│   ├── jobs_sitemap.xml        # Cached sitemap with job listing URLs
│   └── launches_sitemap.xml    # Cached sitemap with startup launch URLs
├── db/
│   ├── __init__.py
│   ├── schema.py               # SQLite schema + connection helpers
│   └── yc.sqlite               # Database storing URLs and scraped content
├── scraper/
│   ├── __init__.py
│   ├── check_remaining.py      # CLI to check scraping progress
│   ├── sitemap_parser.py       # Fetches sitemaps and loads URLs into DB
│   └── yc_scraper.py           # Main scraper - runs batch pipeline
└── README.md
```

## Setup

```bash
pip install desync-search
export DESYNC_API_KEY="your_key"
python db/schema.py  # Initialize database
```

## Usage

```bash
# 1. Load URLs from sitemaps
python scraper/sitemap_parser.py          # Fetch live sitemaps
python scraper/sitemap_parser.py --local  # Use cached XML files

# 2. Scrape pages
python scraper/yc_scraper.py                        # Scrape all
python scraper/yc_scraper.py --pattern /companies/  # Filter by URL
python scraper/yc_scraper.py --stats                # View stats only

# 3. Check progress
python scraper/check_remaining.py /companies/
```

## Files

| File | Purpose |
|------|---------|
| `db/schema.py` | Defines two tables: `websites_from_sitemap` (URLs to scrape) and `pagedataobjects` (scraped content) |
| `scraper/sitemap_parser.py` | Parses YC sitemaps (companies, jobs, launches, library, main) and inserts URLs into the database |
| `scraper/yc_scraper.py` | Runs the scrape pipeline: fetch unvisited URLs → bulk search via Desync → save results → mark visited |
| `scraper/check_remaining.py` | Quick utility to see how many URLs are left to scrape |