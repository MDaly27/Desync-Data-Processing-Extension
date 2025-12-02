# processing_tech

Sequential data processor for YC company pages.

## Pipeline

Each pass runs in order, building on the previous:

```
Pass 1: Companies   -> header + sidebar fields  -> companies table
Pass 2: Tags        -> ALL_CAPS from header     -> tags table
Pass 3: Founders    -> founders section         -> founders table
Pass 4: News        -> news section             -> news table
Pass 5: Links       -> external_links JSON      -> links table (with founder matching)
```

## Page Structure

The YC company pages have a consistent structure:

```
[Nav]
Companies › [Name]
[Tagline]
[SEASON YEAR]
[STATUS]
[TAG1]
[TAG2]
...
[LOCATION]
Company
Jobs
[COUNT]
[Description]
---
Latest News (optional)
[Title] - [Source]
[Date]
---
Founders / Active Founders / Former Founders
[Name]
 
[Title]
---
[Sidebar]
Founded: [YEAR]
Team Size: [NUMBER]
Primary Partner: [NAME]
---
Footer
```

## Files

```
src/
├── main.rs            # Orchestrates passes 1-5
├── db.rs              # Connection, generic insert, table creation
├── pass1_companies.rs # Extract header + sidebar -> companies
├── pass2_tags.rs      # Extract ALL_CAPS -> tags (excludes status/season/location)
├── pass3_founders.rs  # Extract founders -> founders
├── pass4_news.rs      # Extract news -> news
└── pass5_links.rs     # Classify links, match to founders -> links

schema.json            # Table definitions, section markers, extraction rules
```

## Usage

```bash
# From this directory
cargo run

# Or specify database path
YC_DB_PATH=../scraping_tech/data/yc.sqlite cargo run
```

## Schema (from schema.json)

**companies** (slug is primary key)
- slug, name, tagline
- batch_season, batch_year
- status, location
- founded_year, team_size, primary_partner
- job_count, source_url

**tags** (company_slug, tag)

**founders** (company_slug, name, title)

**news** (company_slug, title, source, published_date)

**links** (company_slug, founder_id, url, pattern)
- founder_id is NULL for company links, set for founder-specific links
- pattern is URL pattern like "linkedin.com/in", "twitter.com", etc.

## Key Decisions

1. **Slug as primary key** - No UUIDs, the URL slug is unique
2. **Sequential passes** - Each writes to DB before next starts, saves memory
3. **Section markers** - Known start/stop patterns define where to extract
4. **Founder link matching** - Pass 5 uses founder names from pass 3 to match personal links