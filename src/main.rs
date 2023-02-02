use console::{Key, Term};
use reqwest::blocking;
use scraper::{Html, Selector};
use std::io::Write;

mod story;
mod tags;
mod view;

use story::Story;
use tags::Tag;
use view::{Travel, View, ViewMode};

const URL: &str = "https://lobste.rs";
const PAGE_URL: &str = "https://lobste.rs/page";
const STORIES_PER_SITE_PAGE: usize = 25;

fn get_page(url: String) -> Result<Html, Box<dyn std::error::Error>> {
    let client = blocking::Client::new();
    #[cfg(debug_assertions)]
    eprint!("loading ({url}) ...");
    #[cfg(not(debug_assertions))]
    eprint!("loading ...");
    let res = client
        .get(url)
        .header(
            "user-agent",
            "Mozilla/5.0 (X11; Linux x86_64; rv:12.0) Gecko/20100101 Firefox/12.0",
        )
        .send()?;
    let status = res.error_for_status_ref()?.status();
    #[cfg(debug_assertions)]
    eprintln!("\u{1b}[2K\rloaded ({status})");

    let html = scraper::Html::parse_document(&res.text()?);
    Ok(html)
}

fn get_stories(page: u16) -> Result<Vec<Story>, Box<dyn std::error::Error>> {
    let url = format!("{PAGE_URL}/{page}");
    let html = get_page(url)?;
    let stories_selector = Selector::parse("ol.stories > .story > .story_liner").unwrap();
    let stories_list = html.select(&stories_selector);
    Ok(stories_list.map(Story::from_html).collect())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut term = Term::stdout();
    term.set_title("kreeftje");
    term.hide_cursor()?;
    print!("\u{1b}[?1049h"); // Open the alternative screen buffer.
    term.clear_screen()?;

    let (rows, columns) = console::Term::stdout().size();
    let mut view = View::new(rows as usize / 3);

    'listen: loop {
        view.load_stories_next_page()?;
        if let ViewMode::Comments = view.mode() {
            let story = view.get_selected_story_mut();

            if story.comments_descendants() != story.comments_number() {
                story.load_comments()?
            }
        }
        term.move_cursor_to(0, 0)?;
        term.clear_line()?;

        let view_string = view.generate_string(columns, rows);
        #[cfg(debug_assertions)]
        term.write_fmt(format_args!(
            "page {} (list_pos: {}, comments_pos: {}, stored comments count: {})\n",
            view.site_page(),
            view.pos(),
            view.comments_pos,
            view.get_selected_story().comments_descendants()
        ))?;
        #[cfg(not(debug_assertions))]
        term.write_fmt(format_args!("page {}\n", view.site_page()))?;
        term.write_all(view_string.as_bytes())?;
        let input = term.read_key()?;

        let prev_site_page = view.view_page();

        match input {
            // J — vv
            // Load next page.
            Key::Char('J') => view.go_to(Travel::NextStep),
            // K — ^^
            // Load previous page.
            Key::Char('K') => view.go_to(Travel::PrevStep),
            // j — v
            // Select next story.
            Key::Char('j') | Key::ArrowDown => view.go_to(Travel::NextItem),
            // k — ^
            // Select other story.
            Key::Char('k') | Key::ArrowUp => view.go_to(Travel::PrevItem),
            // l — >
            // Open comments.
            Key::Char('l') | Key::ArrowRight => view.view_comments(),
            // h — <
            // Close comments.
            Key::Char('h') | Key::ArrowLeft => view.view_list(),
            // Toggle comments.
            Key::Char('c') | Key::Tab => view.view_toggle(),
            // g — ^^
            // Go to first page.
            Key::Char('g') => view.go_to(Travel::Top),
            // G — vv
            // Go to last loaded page.
            Key::Char('G') => view.go_to(Travel::Bottom),
            // Open link in browser.
            Key::Char('o') | Key::Enter => webbrowser::open(view.get_selected_story().url())?,

            // Quit
            Key::Char('q') => {
                // q to quit (<ctrl-C> als works, of course)
                break 'listen;
            }

            // Otherwise, do nothing
            _ => {}
        }

        if view.mode() == ViewMode::Comments || prev_site_page != view.view_page() {
            term.clear_screen()?;
        }
        term.move_cursor_to(0, 0)?;
        term.clear_line()?;
    }

    term.clear_screen()?;
    term.show_cursor()?;
    print!("\u{1b}[?1049l"); // Close the alternative screen buffer again.

    Ok(())
}
