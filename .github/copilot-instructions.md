# Copilot Instructions for Bible Headlines Project

## Project Overview

A modern web application built in **Rust** that displays Bible section headings for all 66 books of the Bible. The application focuses on providing clean, searchable access to book names, chapter numbers, and section headings—**without displaying verse text**.

## Technical Stack

- **Backend**: Rust with Actix-web framework (v4.9)
- **Frontend**: Server-rendered HTML with vanilla JavaScript
- **Styling**: Modern CSS with responsive design
- **Fonts**: Cormorant Garamond (headings) & Inter (body text)
- **Server**: Localhost port 8080

## Project Structure

```
bible-headlines/
├── Cargo.toml              # Rust dependencies
├── src/
│   └── main.rs            # Main application code
├── docs/                  # GitHub Pages deployment
│   ├── index.html        # Static HTML export from Rust server
│   ├── data.json         # JSON export (optional, for reference)
│   └── README.md         # Deployment instructions
├── README.md              # User documentation
└── .github/
    └── copilot-instructions.md  # This file
```

## Data Structure

### Core Types

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
struct SectionHeading {
    heading: String,  // E.g., "The Creation of the World"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Chapter {
    chapter_number: u32,
    sections: Vec<SectionHeading>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
enum Testament {
    Old,
    New,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Book {
    name: String,           // E.g., "Genesis"
    testament: Testament,   // Old or New
    chapters: Vec<Chapter>,
}
```

## Design Principles

### UX/UI Guidelines

1. **Sleek & Classy**: Modern gradient backgrounds (purple: #667eea to #764ba2)
2. **Mobile-First**: Fully responsive design with touch-optimized controls
3. **Easy Navigation**: 
   - Collapsible book sections (start collapsed)
   - Real-time search filtering
   - Testament filtering (All/Old/New)
4. **Reverent Presentation**: Respectful display of sacred text
5. **Clean Typography**: Large readable fonts with proper hierarchy
6. **Smooth Interactions**: Transitions on hover, expand/collapse

### Color Palette

- **Primary Gradient**: `linear-gradient(135deg, #667eea 0%, #764ba2 100%)`
- **Text Primary**: `#2d3748`
- **Text Secondary**: `#4a5568`
- **Accent**: `#667eea`
- **Background White**: `#ffffff`
- **Light Background**: `#f6f8fb to #ffffff`
- **Border**: `#e2e8f0`

### Responsive Breakpoints

- **Mobile**: < 768px (single column layout)
- **Desktop**: ≥ 768px (multi-column grid)

## Key Features

### 1. Complete Bible Coverage

- **Old Testament**: 39 books (Genesis through Malachi)
- **New Testament**: 27 books (Matthew through Revelation)
- **Total**: 66 books with proper chapter counts

### 2. Search Functionality

- Real-time filtering across books, chapters, and section headings
- Case-insensitive search
- Automatically expands matching books
- Shows result count statistics

### 3. Testament Filtering

- Filter by All Books, Old Testament, or New Testament
- Visual button states (active/inactive)
- Combines with search for powerful navigation

### 4. Collapsible Interface

- Books start collapsed to avoid overwhelming display
- Click book headers to expand/collapse
- Smooth animations with visual indicators (▼ icon)

### 5. REST API

- `GET /` - Main web interface
- `GET /api/books` - JSON API returning complete book data

## Adding New Content

### Method 1: Adding Detailed Sections to Existing Books

Currently, 7 books have detailed sections:
1. Genesis
2. Exodus
3. Psalms
4. Matthew
5. John
6. Romans
7. Revelation

To add details to placeholder books (e.g., Leviticus), replace the placeholder:

```rust
// Change from:
create_placeholder_book("Leviticus", Testament::Old, 27),

// To:
Book {
    name: "Leviticus".to_string(),
    testament: Testament::Old,
    chapters: vec![
        Chapter {
            chapter_number: 1,
            sections: vec![
                SectionHeading { 
                    heading: "The Burnt Offering".to_string() 
                },
            ],
        },
        Chapter {
            chapter_number: 2,
            sections: vec![
                SectionHeading { 
                    heading: "The Grain Offering".to_string() 
                },
            ],
        },
        // ... continue for all 27 chapters
    ],
},
```

### Method 2: Adding Multiple Sections Per Chapter

Chapters can have multiple section headings:

```rust
Chapter {
    chapter_number: 5,
    sections: vec![
        SectionHeading { heading: "The Sermon on the Mount".to_string() },
        SectionHeading { heading: "The Beatitudes".to_string() },
        SectionHeading { heading: "Salt and Light".to_string() },
    ],
},
```

### Method 3: Updating Placeholder Text

The placeholder function creates chapters with generic text:

```rust
fn create_placeholder_book(name: &str, testament: Testament, chapter_count: u32) -> Book {
    Book {
        name: name.to_string(),
        testament,
        chapters: (1..=chapter_count)
            .map(|num| Chapter {
                chapter_number: num,
                sections: vec![
                    SectionHeading {
                        heading: format!("Section headings to be added"),
                    },
                ],
            })
            .collect(),
    }
}
```

You can update this placeholder text or gradually replace placeholders with actual content.

## Development Workflow

### Building the Project

```bash
cd /home/jcabr/chamorrowizard/projects/bible-headlines
cargo build
```

### Running the Server

```bash
cargo run
```

Server starts at: `http://localhost:8080`

### Stopping the Server

```bash
lsof -ti:8080 | xargs kill -9
```

### Testing Changes

1. Update code in `src/main.rs`
2. Stop existing server (if running)
3. Run `cargo build` to check for errors
4. Run `cargo run` to start the updated server
5. Test in browser at `http://localhost:8080`

### Deploying to GitHub Pages

The site is deployed as a static HTML page on GitHub Pages.

**Setup (one-time)**:
1. Go to repository Settings → Pages
2. Set Source to: Branch `main`, Folder `/docs`
3. Save and wait 1-2 minutes for initial deployment

**Updating the deployed site**:

```bash
# 1. Make sure your Rust server is running
cargo run

# 2. Export the complete HTML to docs folder
curl http://localhost:8080 > docs/index.html

# 3. Commit and push to GitHub
git add docs/index.html
git commit -m "Update Bible content"
git push
```

**Live URL**: https://chamorrowizard.github.io/bible-headlines/

GitHub Pages automatically rebuilds within 1-2 minutes after pushing changes.

**Important**: The `docs/index.html` file should always be the exact HTML output from your Rust server to ensure the live site matches your local development version.

## Code Organization

### Main Function Flow

1. **Data Loading**: `get_bible_data()` creates all book structures
2. **State Setup**: Data wrapped in `Arc` for thread-safe sharing
3. **Server Launch**: Actix-web HTTP server on port 8080
4. **Routes**:
   - `/` → `index()` → `generate_html()` → Full HTML page
   - `/api/books` → `api_books()` → JSON response

### HTML Generation

The `generate_html()` function:
1. Iterates through all books
2. Creates collapsible book cards with testament classes
3. Generates chapter grids with search metadata
4. Embeds JavaScript for interactivity
5. Returns complete HTML document

### JavaScript Functionality

**Key Functions**:
- `toggleBook(header)` - Expand/collapse book sections
- `filterTestament(testament)` - Filter by Old/New Testament
- `filterContent()` - Search and filter logic
- Initialization - Collapse all books on page load

## Biblical Book Order

### Old Testament (39 books)

**Law (Pentateuch)**:
Genesis, Exodus, Leviticus, Numbers, Deuteronomy

**History**:
Joshua, Judges, Ruth, 1 Samuel, 2 Samuel, 1 Kings, 2 Kings, 1 Chronicles, 2 Chronicles, Ezra, Nehemiah, Esther

**Poetry/Wisdom**:
Job, Psalms, Proverbs, Ecclesiastes, Song of Solomon

**Major Prophets**:
Isaiah, Jeremiah, Lamentations, Ezekiel, Daniel

**Minor Prophets**:
Hosea, Joel, Amos, Obadiah, Jonah, Micah, Nahum, Habakkuk, Zephaniah, Haggai, Zechariah, Malachi

### New Testament (27 books)

**Gospels**:
Matthew, Mark, Luke, John

**History**:
Acts

**Paul's Letters**:
Romans, 1 Corinthians, 2 Corinthians, Galatians, Ephesians, Philippians, Colossians, 1 Thessalonians, 2 Thessalonians, 1 Timothy, 2 Timothy, Titus, Philemon

**General Letters**:
Hebrews, James, 1 Peter, 2 Peter, 1 John, 2 John, 3 John, Jude

**Apocalyptic**:
Revelation

## Correct Chapter Counts by Book

When adding content, ensure correct chapter counts:

| Book | Chapters | Book | Chapters |
|------|----------|------|----------|
| Genesis | 50 | Matthew | 28 |
| Exodus | 40 | Mark | 16 |
| Leviticus | 27 | Luke | 24 |
| Numbers | 36 | John | 21 |
| Deuteronomy | 34 | Acts | 28 |
| Joshua | 24 | Romans | 16 |
| Judges | 21 | 1 Corinthians | 16 |
| Ruth | 4 | 2 Corinthians | 13 |
| 1 Samuel | 31 | Galatians | 6 |
| 2 Samuel | 24 | Ephesians | 6 |
| 1 Kings | 22 | Philippians | 4 |
| 2 Kings | 25 | Colossians | 4 |
| 1 Chronicles | 29 | 1 Thessalonians | 5 |
| 2 Chronicles | 36 | 2 Thessalonians | 3 |
| Ezra | 10 | 1 Timothy | 6 |
| Nehemiah | 13 | 2 Timothy | 4 |
| Esther | 10 | Titus | 3 |
| Job | 42 | Philemon | 1 |
| Psalms | 150 | Hebrews | 13 |
| Proverbs | 31 | James | 5 |
| Ecclesiastes | 12 | 1 Peter | 5 |
| Song of Solomon | 8 | 2 Peter | 3 |
| Isaiah | 66 | 1 John | 5 |
| Jeremiah | 52 | 2 John | 1 |
| Lamentations | 5 | 3 John | 1 |
| Ezekiel | 48 | Jude | 1 |
| Daniel | 12 | Revelation | 22 |
| Hosea | 14 | | |
| Joel | 3 | | |
| Amos | 9 | | |
| Obadiah | 1 | | |
| Jonah | 4 | | |
| Micah | 7 | | |
| Nahum | 3 | | |
| Habakkuk | 3 | | |
| Zephaniah | 3 | | |
| Haggai | 2 | | |
| Zechariah | 14 | | |
| Malachi | 4 | | |

**Note**: Current placeholder data has simplified chapter counts for some books (e.g., Genesis has 4 instead of 50). Update these as you add detailed content.

## Future Enhancement Ideas

1. **Verse Navigation**: Add ability to show verse ranges for each section
2. **Cross-References**: Link related sections across books
3. **Themes/Tags**: Categorize sections by themes (salvation, prayer, wisdom, etc.)
4. **Bookmarks**: Let users save favorite sections
5. **Reading Plans**: Guided reading through sections
6. **Dark Mode**: Alternative color scheme
7. **Export**: PDF or print-friendly views
8. **Translations**: Support different Bible versions/translations
9. **Notes**: User annotations on sections
10. **Share**: Social media or link sharing

## Best Practices

### When Adding Content

1. **Accuracy**: Use standard section headings from reputable Bible versions (ESV, NIV, NRSV, etc.)
2. **Consistency**: Keep heading style consistent across books
3. **Completeness**: Try to add all chapters for a book, not just select ones
4. **Testing**: Always test after major changes to ensure search and filtering still work

### Code Style

1. Use descriptive variable names
2. Keep functions focused and small
3. Add comments for complex logic
4. Follow Rust naming conventions (snake_case for functions/variables)
5. Run `cargo fmt` to format code
6. Run `cargo clippy` for linting

### Performance

- Data is loaded once at startup (no database calls)
- Search is client-side JavaScript (instant results)
- HTML generation is fast (server-side rendering)
- No external API calls needed

## Troubleshooting

### Port Already in Use

```bash
lsof -ti:8080 | xargs kill -9
```

### Compilation Errors

- Check syntax (especially vec![] brackets)
- Ensure Testament enum matches exactly: `Testament::Old` or `Testament::New`
- Verify all strings use `.to_string()` or `String::from()`

### Search Not Working

- Ensure `data-search-text` attribute is populated in HTML
- Check JavaScript console for errors
- Verify search box has `id="searchBox"`

### Books Not Collapsing

- Check that JavaScript is executing on page load
- Verify `.collapsed` CSS class is defined
- Ensure `onclick` handlers are properly set

## Project Philosophy

This application is designed as a **reference tool** and **study aid** for Bible readers. It emphasizes:

- **Clarity over complexity**: Simple, focused interface
- **Accessibility**: Works on all devices and browsers
- **Respect**: Reverent presentation of Scripture
- **Extensibility**: Easy to add more content over time
- **Performance**: Fast, no external dependencies
- **Self-contained**: No database, no auth, just pure content delivery

## Contact & Contribution

This is a personal study project. When adding content:
- Focus on accuracy and consistency
- Test thoroughly on mobile and desktop
- Keep the design clean and classy
- Maintain the reverent tone

---

**Last Updated**: February 14, 2026
**Rust Edition**: 2021
**Actix-web Version**: 4.9
