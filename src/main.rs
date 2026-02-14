use actix_web::{web, App, HttpServer, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SectionHeading {
    heading: String,
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
    name: String,
    testament: Testament,
    chapters: Vec<Chapter>,
}

#[derive(Clone)]
struct AppState {
    books: Arc<Vec<Book>>,
}

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

fn get_bible_data() -> Vec<Book> {
    vec![
        // OLD TESTAMENT - Detailed Books
        Book {
            name: "Genesis".to_string(),
            testament: Testament::Old,
            chapters: vec![
                Chapter {
                    chapter_number: 1,
                    sections: vec![
                        SectionHeading { heading: "The Creation of the World".to_string() },
                    ],
                },
                Chapter {
                    chapter_number: 2,
                    sections: vec![
                        SectionHeading { heading: "The Seventh Day, God Rests".to_string() },
                        SectionHeading { heading: "The Creation of Man and Woman".to_string() },
                    ],
                },
                Chapter {
                    chapter_number: 3,
                    sections: vec![
                        SectionHeading { heading: "The Fall".to_string() },
                    ],
                },
                Chapter {
                    chapter_number: 4,
                    sections: vec![
                        SectionHeading { heading: "Cain and Abel".to_string() },
                    ],
                },
            ],
        },
        Book {
            name: "Exodus".to_string(),
            testament: Testament::Old,
            chapters: vec![
                Chapter {
                    chapter_number: 1,
                    sections: vec![
                        SectionHeading { heading: "Israel Increases Greatly in Egypt".to_string() },
                    ],
                },
                Chapter {
                    chapter_number: 2,
                    sections: vec![
                        SectionHeading { heading: "The Birth of Moses".to_string() },
                        SectionHeading { heading: "Moses Flees to Midian".to_string() },
                    ],
                },
                Chapter {
                    chapter_number: 3,
                    sections: vec![
                        SectionHeading { heading: "The Burning Bush".to_string() },
                    ],
                },
                Chapter {
                    chapter_number: 14,
                    sections: vec![
                        SectionHeading { heading: "Crossing the Red Sea".to_string() },
                    ],
                },
                Chapter {
                    chapter_number: 20,
                    sections: vec![
                        SectionHeading { heading: "The Ten Commandments".to_string() },
                    ],
                },
            ],
        },
        Book {
            name: "Psalms".to_string(),
            testament: Testament::Old,
            chapters: vec![
                Chapter {
                    chapter_number: 1,
                    sections: vec![
                        SectionHeading { heading: "The Way of the Righteous and the Wicked".to_string() },
                    ],
                },
                Chapter {
                    chapter_number: 23,
                    sections: vec![
                        SectionHeading { heading: "The Lord Is My Shepherd".to_string() },
                    ],
                },
                Chapter {
                    chapter_number: 91,
                    sections: vec![
                        SectionHeading { heading: "My Refuge and My Fortress".to_string() },
                    ],
                },
                Chapter {
                    chapter_number: 139,
                    sections: vec![
                        SectionHeading { heading: "You Have Searched Me and Known Me".to_string() },
                    ],
                },
            ],
        },
        
        // OLD TESTAMENT - Remaining Books (Placeholders)
        create_placeholder_book("Leviticus", Testament::Old, 27),
        create_placeholder_book("Numbers", Testament::Old, 36),
        create_placeholder_book("Deuteronomy", Testament::Old, 34),
        create_placeholder_book("Joshua", Testament::Old, 24),
        create_placeholder_book("Judges", Testament::Old, 21),
        create_placeholder_book("Ruth", Testament::Old, 4),
        create_placeholder_book("1 Samuel", Testament::Old, 31),
        create_placeholder_book("2 Samuel", Testament::Old, 24),
        create_placeholder_book("1 Kings", Testament::Old, 22),
        create_placeholder_book("2 Kings", Testament::Old, 25),
        create_placeholder_book("1 Chronicles", Testament::Old, 29),
        create_placeholder_book("2 Chronicles", Testament::Old, 36),
        create_placeholder_book("Ezra", Testament::Old, 10),
        create_placeholder_book("Nehemiah", Testament::Old, 13),
        create_placeholder_book("Esther", Testament::Old, 10),
        create_placeholder_book("Job", Testament::Old, 42),
        create_placeholder_book("Proverbs", Testament::Old, 31),
        create_placeholder_book("Ecclesiastes", Testament::Old, 12),
        create_placeholder_book("Song of Solomon", Testament::Old, 8),
        create_placeholder_book("Isaiah", Testament::Old, 66),
        create_placeholder_book("Jeremiah", Testament::Old, 52),
        create_placeholder_book("Lamentations", Testament::Old, 5),
        create_placeholder_book("Ezekiel", Testament::Old, 48),
        create_placeholder_book("Daniel", Testament::Old, 12),
        create_placeholder_book("Hosea", Testament::Old, 14),
        create_placeholder_book("Joel", Testament::Old, 3),
        create_placeholder_book("Amos", Testament::Old, 9),
        create_placeholder_book("Obadiah", Testament::Old, 1),
        create_placeholder_book("Jonah", Testament::Old, 4),
        create_placeholder_book("Micah", Testament::Old, 7),
        create_placeholder_book("Nahum", Testament::Old, 3),
        create_placeholder_book("Habakkuk", Testament::Old, 3),
        create_placeholder_book("Zephaniah", Testament::Old, 3),
        create_placeholder_book("Haggai", Testament::Old, 2),
        create_placeholder_book("Zechariah", Testament::Old, 14),
        create_placeholder_book("Malachi", Testament::Old, 4),
        
        // NEW TESTAMENT - Detailed Books
        Book {
            name: "Matthew".to_string(),
            testament: Testament::New,
            chapters: vec![
                Chapter {
                    chapter_number: 1,
                    sections: vec![
                        SectionHeading { heading: "Superscription: The Messianic Thesis Statement (1:1)".to_string() },
                        SectionHeading { heading: "Abraham to David: Covenant Line through the Patriarchs (1:2–6a)".to_string() },
                        SectionHeading { heading: "David to the Exile: Royal Decline and Judgment (1:6b–11)".to_string() },
                        SectionHeading { heading: "Exile to Messiah: Restoration and Fulfillment (1:12–16)".to_string() },
                        SectionHeading { heading: "Theological Structuring of Israel's History (1:17)".to_string() },
                        SectionHeading { heading: "Conception by the Holy Spirit (1:18)".to_string() },
                        SectionHeading { heading: "Joseph's Righteousness and Intended Mercy (1:19)".to_string() },
                        SectionHeading { heading: "Angelic Revelation: Divine Initiative Explained (1:20–21)".to_string() },
                        SectionHeading { heading: "Prophetic Fulfillment Citation (1:22–23)".to_string() },
                        SectionHeading { heading: "Obedient Response of Joseph (1:24–25a)".to_string() },
                        SectionHeading { heading: "Naming the Child (1:25b)".to_string() },
                    ],
                },
                Chapter {
                    chapter_number: 2,
                    sections: vec![
                        SectionHeading { heading: "The Birth in Bethlehem and the Arrival of the Magi (2:1–2)".to_string() },
                        SectionHeading { heading: "Herod's Alarm and Prophetic Clarification (2:3–6)".to_string() },
                        SectionHeading { heading: "Herod's Deceptive Inquiry (2:7–8)".to_string() },
                        SectionHeading { heading: "The Star's Guidance and Joyful Confirmation (2:9–10)".to_string() },
                        SectionHeading { heading: "Homage and Royal Gifts (2:11)".to_string() },
                        SectionHeading { heading: "Divine Warning and Providential Protection (2:12)".to_string() },
                        SectionHeading { heading: "Escape to Egypt and Fulfillment of Scripture (2:13–15)".to_string() },
                        SectionHeading { heading: "The Massacre of the Infants (2:16–18)".to_string() },
                        SectionHeading { heading: "Death of Herod and Return from Egypt (2:19–21)".to_string() },
                        SectionHeading { heading: "Fear of Archelaus (2:22)".to_string() },
                        SectionHeading { heading: "Settlement in Nazareth and Prophetic Fulfillment (2:23)".to_string() },
                    ],
                },
                Chapter {
                    chapter_number: 5,
                    sections: vec![
                        SectionHeading { heading: "The Sermon on the Mount".to_string() },
                        SectionHeading { heading: "The Beatitudes".to_string() },
                    ],
                },
                Chapter {
                    chapter_number: 6,
                    sections: vec![
                        SectionHeading { heading: "Giving to the Needy".to_string() },
                        SectionHeading { heading: "The Lord's Prayer".to_string() },
                    ],
                },
                Chapter {
                    chapter_number: 28,
                    sections: vec![
                        SectionHeading { heading: "The Resurrection".to_string() },
                        SectionHeading { heading: "The Great Commission".to_string() },
                    ],
                },
            ],
        },
        create_placeholder_book("Mark", Testament::New, 16),
        create_placeholder_book("Luke", Testament::New, 24),
        Book {
            name: "John".to_string(),
            testament: Testament::New,
            chapters: vec![
                Chapter {
                    chapter_number: 1,
                    sections: vec![
                        SectionHeading { heading: "The Word Became Flesh".to_string() },
                        SectionHeading { heading: "The Testimony of John the Baptist".to_string() },
                    ],
                },
                Chapter {
                    chapter_number: 3,
                    sections: vec![
                        SectionHeading { heading: "You Must Be Born Again".to_string() },
                        SectionHeading { heading: "For God So Loved the World".to_string() },
                    ],
                },
                Chapter {
                    chapter_number: 11,
                    sections: vec![
                        SectionHeading { heading: "The Death of Lazarus".to_string() },
                        SectionHeading { heading: "I Am the Resurrection and the Life".to_string() },
                    ],
                },
                Chapter {
                    chapter_number: 14,
                    sections: vec![
                        SectionHeading { heading: "I Am the Way, the Truth, and the Life".to_string() },
                    ],
                },
            ],
        },
        create_placeholder_book("Acts", Testament::New, 28),
        Book {
            name: "Romans".to_string(),
            testament: Testament::New,
            chapters: vec![
                Chapter {
                    chapter_number: 1,
                    sections: vec![
                        SectionHeading { heading: "Greeting".to_string() },
                        SectionHeading { heading: "The Righteous Shall Live by Faith".to_string() },
                    ],
                },
                Chapter {
                    chapter_number: 3,
                    sections: vec![
                        SectionHeading { heading: "No One Is Righteous".to_string() },
                        SectionHeading { heading: "Righteousness Through Faith in Christ".to_string() },
                    ],
                },
                Chapter {
                    chapter_number: 5,
                    sections: vec![
                        SectionHeading { heading: "Peace with God Through Faith".to_string() },
                    ],
                },
                Chapter {
                    chapter_number: 8,
                    sections: vec![
                        SectionHeading { heading: "Life in the Spirit".to_string() },
                        SectionHeading { heading: "More Than Conquerors".to_string() },
                    ],
                },
                Chapter {
                    chapter_number: 12,
                    sections: vec![
                        SectionHeading { heading: "A Living Sacrifice".to_string() },
                        SectionHeading { heading: "Gifts of Grace".to_string() },
                    ],
                },
            ],
        },
        create_placeholder_book("1 Corinthians", Testament::New, 16),
        create_placeholder_book("2 Corinthians", Testament::New, 13),
        create_placeholder_book("Galatians", Testament::New, 6),
        create_placeholder_book("Ephesians", Testament::New, 6),
        create_placeholder_book("Philippians", Testament::New, 4),
        create_placeholder_book("Colossians", Testament::New, 4),
        create_placeholder_book("1 Thessalonians", Testament::New, 5),
        create_placeholder_book("2 Thessalonians", Testament::New, 3),
        create_placeholder_book("1 Timothy", Testament::New, 6),
        create_placeholder_book("2 Timothy", Testament::New, 4),
        create_placeholder_book("Titus", Testament::New, 3),
        create_placeholder_book("Philemon", Testament::New, 1),
        create_placeholder_book("Hebrews", Testament::New, 13),
        create_placeholder_book("James", Testament::New, 5),
        create_placeholder_book("1 Peter", Testament::New, 5),
        create_placeholder_book("2 Peter", Testament::New, 3),
        create_placeholder_book("1 John", Testament::New, 5),
        create_placeholder_book("2 John", Testament::New, 1),
        create_placeholder_book("3 John", Testament::New, 1),
        create_placeholder_book("Jude", Testament::New, 1),
        Book {
            name: "Revelation".to_string(),
            testament: Testament::New,
            chapters: vec![
                Chapter {
                    chapter_number: 1,
                    sections: vec![
                        SectionHeading { heading: "Prologue".to_string() },
                        SectionHeading { heading: "Vision of the Son of Man".to_string() },
                    ],
                },
                Chapter {
                    chapter_number: 21,
                    sections: vec![
                        SectionHeading { heading: "The New Heaven and the New Earth".to_string() },
                    ],
                },
                Chapter {
                    chapter_number: 22,
                    sections: vec![
                        SectionHeading { heading: "The River of Life".to_string() },
                        SectionHeading { heading: "Jesus Is Coming".to_string() },
                    ],
                },
            ],
        },
    ]
}

async fn index(data: web::Data<AppState>) -> Result<HttpResponse> {
    let html = generate_html(&data.books);
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html))
}

async fn api_books(data: web::Data<AppState>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(&*data.books))
}

fn generate_html(books: &[Book]) -> String {
    let mut content = String::new();
    
    for book in books {
        let testament_class = match book.testament {
            Testament::Old => "old-testament",
            Testament::New => "new-testament",
        };
        
        content.push_str(&format!(r#"
            <div class="book-card {}" data-book-name="{}" data-testament="{}">
                <div class="book-header" onclick="toggleBook(this)">
                    <h2 class="book-title">{}</h2>
                    <span class="expand-icon">▼</span>
                </div>
                <div class="chapters-grid">
        "#, testament_class, book.name.to_lowercase(), testament_class, book.name));
        
        for chapter in &book.chapters {
            let mut search_text = String::new();
            for section in &chapter.sections {
                search_text.push_str(&format!("{} ", section.heading.to_lowercase()));
            }
            
            content.push_str(&format!(r#"
                <div class="chapter-card" data-search-text="{}">
                    <div class="chapter-header">
                        <span class="chapter-number">Chapter {}</span>
                    </div>
                    <div class="sections">
            "#, search_text, chapter.chapter_number));
            
            for section in &chapter.sections {
                content.push_str(&format!(r#"
                        <div class="section-heading">
                            <span class="bullet">•</span>
                            <span class="heading-text">{}</span>
                        </div>
                "#, section.heading));
            }
            
            content.push_str(r#"
                    </div>
                </div>
            "#);
        }
        
        content.push_str(r#"
                </div>
            </div>
        "#);
    }
    
    format!(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Bible Section Headlines</title>
    <link rel="preconnect" href="https://fonts.googleapis.com">
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
    <link href="https://fonts.googleapis.com/css2?family=Cormorant+Garamond:wght@300;400;500;600;700&family=Inter:wght@300;400;500;600&display=swap" rel="stylesheet">
    <style>
        * {{
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }}
        
        body {{
            font-family: 'Inter', sans-serif;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
            padding: 1rem;
            color: #2d3748;
        }}
        
        .container {{
            max-width: 1400px;
            margin: 0 auto;
        }}
        
        header {{
            text-align: center;
            margin-bottom: 2rem;
            padding: 1.5rem;
        }}
        
        h1 {{
            font-family: 'Cormorant Garamond', serif;
            font-size: 3rem;
            font-weight: 700;
            color: #ffffff;
            margin-bottom: 0.5rem;
            text-shadow: 2px 2px 4px rgba(0,0,0,0.1);
            letter-spacing: -1px;
        }}
        
        .subtitle {{
            font-size: 1.1rem;
            color: rgba(255,255,255,0.9);
            font-weight: 300;
            letter-spacing: 0.5px;
            margin-bottom: 1.5rem;
        }}
        
        .controls {{
            background: white;
            border-radius: 16px;
            padding: 1.5rem;
            margin-bottom: 2rem;
            box-shadow: 0 10px 40px rgba(0,0,0,0.1);
            display: flex;
            flex-direction: column;
            gap: 1rem;
        }}
        
        .search-container {{
            position: relative;
            flex: 1;
        }}
        
        .search-box {{
            width: 100%;
            padding: 1rem 1rem 1rem 3rem;
            border: 2px solid #e2e8f0;
            border-radius: 12px;
            font-size: 1rem;
            font-family: 'Inter', sans-serif;
            transition: all 0.3s ease;
            background: #f7fafc;
        }}
        
        .search-box:focus {{
            outline: none;
            border-color: #667eea;
            background: white;
            box-shadow: 0 0 0 3px rgba(102, 126, 234, 0.1);
        }}
        
        .search-icon {{
            position: absolute;
            left: 1rem;
            top: 50%;
            transform: translateY(-50%);
            color: #667eea;
            font-size: 1.2rem;
        }}
        
        .filter-buttons {{
            display: flex;
            gap: 0.75rem;
            flex-wrap: wrap;
        }}
        
        .filter-btn {{
            padding: 0.75rem 1.5rem;
            border: 2px solid #e2e8f0;
            background: white;
            border-radius: 10px;
            font-family: 'Inter', sans-serif;
            font-size: 0.95rem;
            font-weight: 500;
            cursor: pointer;
            transition: all 0.3s ease;
            color: #4a5568;
        }}
        
        .filter-btn:hover {{
            border-color: #667eea;
            color: #667eea;
            transform: translateY(-2px);
        }}
        
        .filter-btn.active {{
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            border-color: transparent;
        }}
        
        .stats {{
            text-align: center;
            color: #4a5568;
            font-size: 0.9rem;
            padding: 0.5rem;
        }}
        
        .book-card {{
            background: white;
            border-radius: 16px;
            margin-bottom: 1.5rem;
            box-shadow: 0 10px 40px rgba(0,0,0,0.1);
            transition: all 0.3s ease;
            overflow: hidden;
        }}
        
        .book-card.hidden {{
            display: none;
        }}
        
        .book-card.collapsed .chapters-grid {{
            display: none;
        }}
        
        .book-card.collapsed .expand-icon {{
            transform: rotate(-90deg);
        }}
        
        .book-header {{
            padding: 1.5rem 2rem;
            cursor: pointer;
            display: flex;
            justify-content: space-between;
            align-items: center;
            transition: background-color 0.2s ease;
        }}
        
        .book-header:hover {{
            background-color: rgba(102, 126, 234, 0.05);
        }}
        
        .book-title {{
            font-family: 'Cormorant Garamond', serif;
            font-size: 2rem;
            font-weight: 700;
            color: #667eea;
            margin: 0;
        }}
        
        .expand-icon {{
            font-size: 1.5rem;
            color: #667eea;
            transition: transform 0.3s ease;
            user-select: none;
        }}
        
        .chapters-grid {{
            display: grid;
            grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
            gap: 1.25rem;
            padding: 1.5rem 2rem 2rem;
        }}
        
        .chapter-card {{
            background: linear-gradient(135deg, #f6f8fb 0%, #ffffff 100%);
            border-radius: 12px;
            padding: 1.25rem;
            border: 1px solid #e2e8f0;
            transition: all 0.3s ease;
            position: relative;
            overflow: hidden;
        }}
        
        .chapter-card.hidden {{
            display: none;
        }}
        
        .chapter-card::before {{
            content: '';
            position: absolute;
            top: 0;
            left: 0;
            width: 4px;
            height: 100%;
            background: linear-gradient(180deg, #667eea 0%, #764ba2 100%);
            opacity: 0;
            transition: opacity 0.3s ease;
        }}
        
        .chapter-card:hover {{
            border-color: #667eea;
            transform: translateX(4px);
        }}
        
        .chapter-card:hover::before {{
            opacity: 1;
        }}
        
        .chapter-header {{
            margin-bottom: 1rem;
        }}
        
        .chapter-number {{
            font-family: 'Cormorant Garamond', serif;
            font-size: 1.3rem;
            font-weight: 600;
            color: #4a5568;
            display: inline-block;
            padding: 0.35rem 0.75rem;
            background: rgba(102, 126, 234, 0.1);
            border-radius: 8px;
        }}
        
        .sections {{
            display: flex;
            flex-direction: column;
            gap: 0.65rem;
        }}
        
        .section-heading {{
            display: flex;
            align-items: flex-start;
            gap: 0.65rem;
            padding: 0.4rem;
            border-radius: 6px;
            transition: background-color 0.2s ease;
        }}
        
        .section-heading:hover {{
            background-color: rgba(102, 126, 234, 0.05);
        }}
        
        .bullet {{
            color: #667eea;
            font-size: 1.3rem;
            line-height: 1.4;
            font-weight: 600;
            flex-shrink: 0;
        }}
        
        .heading-text {{
            font-size: 0.95rem;
            color: #2d3748;
            line-height: 1.5;
            font-weight: 400;
        }}
        
        .highlight {{
            background-color: #fef08a;
            color: #854d0e;
            padding: 0.1rem 0.2rem;
            border-radius: 3px;
            font-weight: 600;
        }}
        
        footer {{
            text-align: center;
            padding: 2rem;
            color: rgba(255,255,255,0.9);
            font-size: 0.9rem;
        }}
        
        @media (max-width: 768px) {{
            h1 {{
                font-size: 2.2rem;
            }}
            
            .subtitle {{
                font-size: 1rem;
            }}
            
            .controls {{
                padding: 1.25rem;
            }}
            
            .filter-buttons {{
                justify-content: center;
            }}
            
            .filter-btn {{
                flex: 1;
                min-width: 100px;
                padding: 0.7rem 1rem;
                font-size: 0.9rem;
            }}
            
            .book-header {{
                padding: 1.25rem 1.5rem;
            }}
            
            .book-title {{
                font-size: 1.6rem;
            }}
            
            .chapters-grid {{
                grid-template-columns: 1fr;
                padding: 1.25rem 1.5rem 1.5rem;
            }}
        }}
        
        @media (min-width: 769px) {{
            .controls {{
                flex-direction: row;
                align-items: center;
            }}
        }}
    </style>
</head>
<body>
    <div class="container">
        <header>
            <h1>✦ Bible Section Headlines ✦</h1>
            <p class="subtitle">Explore all 66 Books of Scripture</p>
        </header>
        
        <div class="controls">
            <div class="search-container">
                <span class="search-icon">🔍</span>
                <input 
                    type="text" 
                    class="search-box" 
                    id="searchBox" 
                    placeholder="Search books, chapters, or section headings..."
                    oninput="filterContent()"
                >
            </div>
            <div class="filter-buttons">
                <button class="filter-btn active" onclick="filterTestament('all')">All Books</button>
                <button class="filter-btn" onclick="filterTestament('old')">Old Testament</button>
                <button class="filter-btn" onclick="filterTestament('new')">New Testament</button>
            </div>
        </div>
        
        <div class="stats" id="stats"></div>
        
        <div class="books-container" id="booksContainer">
            {}
        </div>
        
        <footer>
            <p>A reverent way to explore Scripture • Add detailed sections as you study</p>
        </footer>
    </div>
    
    <script>
        let currentTestament = 'all';
        
        function toggleBook(header) {{
            const bookCard = header.parentElement;
            bookCard.classList.toggle('collapsed');
        }}
        
        function filterTestament(testament) {{
            currentTestament = testament;
            
            // Update button states
            document.querySelectorAll('.filter-btn').forEach(btn => {{
                btn.classList.remove('active');
            }});
            event.target.classList.add('active');
            
            filterContent();
        }}
        
        function highlightText(element, searchTerm) {{
            if (!searchTerm || searchTerm.length < 2) {{
                return;
            }}
            
            const textElements = element.querySelectorAll('.heading-text, .book-title, .chapter-number');
            textElements.forEach(el => {{
                const originalText = el.textContent;
                const regex = new RegExp(`(${{searchTerm.replace(/[.*+?^${{}}()|[\]\\]/g, '\\$&')}})`, 'gi');
                const highlightedText = originalText.replace(regex, '<span class="highlight">$1</span>');
                if (originalText !== highlightedText) {{
                    el.innerHTML = highlightedText;
                }}
            }});
        }}
        
        function clearHighlights() {{
            document.querySelectorAll('.highlight').forEach(el => {{
                const parent = el.parentNode;
                parent.replaceChild(document.createTextNode(el.textContent), el);
            }});
        }}
        
        function filterContent() {{
            const searchTerm = document.getElementById('searchBox').value.toLowerCase();
            const bookCards = document.querySelectorAll('.book-card');
            let visibleBooks = 0;
            let visibleChapters = 0;
            
            // Clear previous highlights
            clearHighlights();
            
            bookCards.forEach(bookCard => {{
                const bookName = bookCard.getAttribute('data-book-name');
                const testament = bookCard.getAttribute('data-testament');
                const chapters = bookCard.querySelectorAll('.chapter-card');
                
                // Testament filter
                let testamentMatch = currentTestament === 'all' || 
                                    testament === currentTestament + '-testament';
                
                // Search filter
                let bookMatches = bookName.includes(searchTerm);
                let hasVisibleChapter = false;
                
                chapters.forEach(chapter => {{
                    const searchText = chapter.getAttribute('data-search-text');
                    const chapterMatches = searchTerm === '' || 
                                          bookMatches || 
                                          searchText.includes(searchTerm);
                    
                    if (chapterMatches && testamentMatch) {{
                        chapter.classList.remove('hidden');
                        hasVisibleChapter = true;
                        visibleChapters++;
                    }} else {{
                        chapter.classList.add('hidden');
                    }}
                }});
                
                if ((bookMatches || hasVisibleChapter) && testamentMatch) {{
                    bookCard.classList.remove('hidden');
                    if (searchTerm !== '') {{
                        bookCard.classList.remove('collapsed');
                        // Highlight matching terms
                        highlightText(bookCard, searchTerm);
                    }}
                    visibleBooks++;
                }} else {{
                    bookCard.classList.add('hidden');
                }}
            }});
            
            // Update stats
            const stats = document.getElementById('stats');
            if (searchTerm === '' && currentTestament === 'all') {{
                stats.textContent = `Showing all 66 books`;
            }} else {{
                stats.textContent = `Found ${{visibleBooks}} book${{visibleBooks !== 1 ? 's' : ''}} with ${{visibleChapters}} chapter${{visibleChapters !== 1 ? 's' : ''}}`;
            }}
        }}
        
        // Initialize: collapse all books
        document.addEventListener('DOMContentLoaded', function() {{
            document.querySelectorAll('.book-card').forEach(card => {{
                card.classList.add('collapsed');
            }});
            filterContent();
        }});
    </script>
</body>
</html>"#, content)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let bible_data = get_bible_data();
    let app_state = web::Data::new(AppState {
        books: Arc::new(bible_data),
    });

    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let bind_address = format!("0.0.0.0:{}", port);

    println!("🚀 Server starting at http://{}", bind_address);
    println!("📖 View Bible Section Headlines");
    
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/", web::get().to(index))
            .route("/api/books", web::get().to(api_books))
    })
    .bind(&bind_address)?
    .run()
    .await
}
