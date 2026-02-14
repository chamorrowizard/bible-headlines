# Bible Section Headlines

A modern, mobile-friendly web application built with Rust that displays Bible section headings for all 66 books in an elegant, classy interface.

## Features

- **All 66 Bible Books**: Complete coverage of Old and New Testament
- **Clean Data Display**: Shows book names, chapter numbers, and section headings without verse text
- **Powerful Search**: Real-time search across books, chapters, and section headings
- **Testament Filtering**: Filter by Old Testament, New Testament, or view all books
- **Collapsible Books**: Click any book to expand/collapse chapters for easy navigation
- **Modern UI Design**: Beautiful purple gradient background with card-based layout
- **Fully Responsive**: Works seamlessly on desktop, tablet, and mobile devices
- **Elegant Typography**: Uses Cormorant Garamond for titles and Inter for body text
- **Smooth Interactions**: Hover effects and smooth transitions throughout
- **REST API**: JSON endpoint available at `/api/books`
- **Expandable Structure**: Easy to add detailed section headings as you study

## Technology Stack

- **Backend**: Rust with Actix-web framework
- **Frontend**: Server-rendered HTML with modern CSS
- **JavaScript**: Client-side search and filtering
- **Data Format**: Structured JSON with proper Rust types

## Running the Application

1. Make sure you have Rust installed (https://rustup.rs/)

2. Build the project:
   ```bash
   cargo build
   ```

3. Run the server:
   ```bash
   cargo run
   ```

4. Open your browser and navigate to:
   ```
   http://localhost:8080
   ```

## Using the Application

### Search
- Type in the search box to instantly filter books and chapters
- Search works across book names, chapter numbers, and section headings
- Matching books automatically expand to show relevant chapters

### Filtering
- **All Books**: View all 66 books of the Bible
- **Old Testament**: Filter to show only the 39 Old Testament books
- **New Testament**: Filter to show only the 27 New Testament books

### Navigation
- Books are collapsed by default for easy browsing
- Click any book header to expand/collapse its chapters
- Smooth animations guide your exploration

## API Endpoints

- `GET /` - Main web interface with beautiful UI
- `GET /api/books` - JSON API returning all Bible books, chapters, and section headings

## Bible Books Included

**Old Testament (39 books)**:
- Law: Genesis, Exodus, Leviticus, Numbers, Deuteronomy
- History: Joshua, Judges, Ruth, 1-2 Samuel, 1-2 Kings, 1-2 Chronicles, Ezra, Nehemiah, Esther
- Wisdom: Job, Psalms, Proverbs, Ecclesiastes, Song of Solomon
- Major Prophets: Isaiah, Jeremiah, Lamentations, Ezekiel, Daniel
- Minor Prophets: Hosea, Joel, Amos, Obadiah, Jonah, Micah, Nahum, Habakkuk, Zephaniah, Haggai, Zechariah, Malachi

**New Testament (27 books)**:
- Gospels: Matthew, Mark, Luke, John
- History: Acts
- Paul's Letters: Romans, 1-2 Corinthians, Galatians, Ephesians, Philippians, Colossians, 1-2 Thessalonians, 1-2 Timothy, Titus, Philemon
- General Letters: Hebrews, James, 1-2 Peter, 1-2-3 John, Jude
- Prophecy: Revelation

## Adding More Detail

The application is designed to easily accommodate detailed section headings:

1. Books like Genesis, Exodus, Psalms, Matthew, John, Romans, and Revelation have sample detailed sections
2. Other books have placeholder sections ready to be filled in
3. Simply edit the book data in `src/main.rs` to add more specific section headings

## Design Philosophy

The application emphasizes:
- **Clarity**: Easy-to-read typography and spacing
- **Elegance**: Sophisticated color palette with purple gradients
- **Accessibility**: Proper contrast, readable font sizes, and mobile-friendly design
- **Efficiency**: Quick search and filtering for instant access
- **Reverence**: Respectful presentation of sacred text
- **Scalability**: Structure supports all books with room to grow

## Mobile Experience

- Optimized touch targets for mobile interaction
- Responsive grid layout adapts to screen size
- Collapsible sections prevent overwhelming on small screens
- Search and filters work seamlessly on touch devices

## Development

Built with expert UX/UI principles following modern web design standards. The codebase is clean, well-structured, and easy to extend with additional books or features.

## License

MIT
