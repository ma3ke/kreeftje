use crate::{get_page, Tag, URL};
use console::style;
use scraper::{ElementRef, Selector};
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub(crate) enum Byline {
    AuthoredBy(String),
    Via(String),
}

impl Display for Byline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Byline::AuthoredBy(user) => format!("authored by {user}"),
                Byline::Via(user) => format!("via {user}"),
            }
        )
    }
}

impl Byline {
    fn from_html(html: ElementRef) -> Self {
        // Take the second element, which contains 'via' or 'authored by'.
        let t = html.text().nth(1).unwrap();

        let user = html
            .select(&Selector::parse(".u-author").unwrap())
            .next()
            .unwrap()
            .text()
            .next()
            .unwrap();

        match t.trim() {
            "via" => Self::Via(user.to_string()),
            "authored by" => Self::AuthoredBy(user.to_string()),
            e => panic!("Cannot parse '{e}' into Byline"),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Comment {
    votes: usize,
    author: String,
    time: String,
    content: String,
    children: Vec<Comment>,
}

impl Comment {
    pub(crate) fn from_html(html: ElementRef) -> Self {
        let s = |s| {
            html.select(&Selector::parse(s).unwrap())
                .next()
                .unwrap()
                .text()
                .next()
        };

        let id = html
            .select(&Selector::parse(".comment").unwrap())
            .next()
            .unwrap()
            .value()
            .id()
            .unwrap();

        Self {
            votes: usize::from_str(s(".comment .voters .score").unwrap()).unwrap_or(0),
            author: html
                .select(&Selector::parse(".comment .details .byline a").unwrap())
                .nth(2)
                .unwrap()
                .text()
                .next()
                .unwrap()
                .to_string(),
            time: html
                .select(&Selector::parse(".comment .details .byline span").unwrap())
                .next()
                .unwrap()
                .text()
                .next()
                .unwrap()
                .to_string(),
            content: html
                .select(&Selector::parse(".comment .details .comment_text").unwrap())
                .next()
                .unwrap()
                .text()
                .map(|element| element.to_string())
                .collect(),
            children: {
                html.select(
                    &Selector::parse(&format!(
                        ".comment#{id} ~ ol.comments > li.comments_subtree"
                    ))
                    .unwrap(),
                )
                .map(Comment::from_html)
                .collect()
            },
        }
    }

    pub fn to_string(&self, width: usize) -> String {
        let indent = "⸾   ";
        let byline = style(format!("({}) {} {}", self.votes, self.author, self.time)).dim();
        let content = wrap(self.content.trim().to_string(), width);
        let children = &self
            .children
            .iter()
            .map(|comment| {
                prepend_string(
                    &comment.to_string(width - indent.len() + 2),
                    &style(indent).dim().to_string(),
                )
            })
            .collect::<Vec<String>>()
            .join("\n");
        format!("{byline}\n{content}\n{children}")
    }

    pub(crate) fn descendants_count(&self) -> usize {
        self.children
            .iter()
            .map(|child| child.descendants_count())
            .sum::<usize>()
            + 1
    }
}

fn wrap(s: String, width: usize) -> String {
    s.lines()
        .map(|line| {
            line.trim_end()
                .chars()
                .collect::<Vec<char>>()
                .chunks(width)
                .map(|chunk| chunk.iter().collect::<String>().trim().to_string())
                .collect::<Vec<String>>()
                .join("\n")
        })
        .collect::<Vec<String>>()
        .join("\n")
}

pub(crate) fn prepend_string(s: &str, prefix: &str) -> String {
    s.lines()
        .map(|line| format!("{prefix}{line}"))
        .collect::<Vec<String>>()
        .join("\n")
}

#[derive(Debug, Clone)]
pub(crate) struct Story {
    votes: usize,
    title: String,
    description: bool,
    tags: Vec<Tag>,
    domain: Option<String>,
    byline: Byline,
    time: String,
    comments_number: usize,
    comments_url: String,
    comments: Vec<Comment>,
    url: String,
}

impl Story {
    pub(crate) fn from_html(html: ElementRef) -> Self {
        let s = |s| {
            html.select(&Selector::parse(s).unwrap())
                .next()
                .unwrap()
                .text()
                .next()
        };

        let description = html
            .select(&Selector::parse(".details > a.description_present").unwrap())
            .next()
            .is_some();

        Self {
            votes: usize::from_str(s(".voters > .score").unwrap()).unwrap(),
            title: s(".details > .link > a").unwrap().to_string(),
            description,
            tags: html
                .select(&Selector::parse(".details > .tags > .tag").unwrap())
                .map(|t| Tag::from_str(t.text().next().unwrap()).unwrap())
                .collect(),
            domain: {
                let anchor = html
                    .select(&Selector::parse(".details > .domain").unwrap())
                    .next();

                anchor.map(|s| s.text().next().unwrap().to_string())
            },
            byline: Byline::from_html(
                html.select(&Selector::parse(".details > .byline").unwrap())
                    .next()
                    .unwrap(),
            ),
            time: html
                .select(&Selector::parse(".details > .byline > span").unwrap())
                .next()
                .unwrap()
                .text()
                .next()
                .unwrap()
                .to_string(),

            comments_number: match s(".details > .byline > .comments_label > a")
                .unwrap()
                .split_whitespace()
                .next()
                .unwrap()
            {
                "no" => 0,
                n => usize::from_str(n).unwrap(),
            },
            comments: Vec::new(),
            comments_url: {
                let url = html
                    .select(&Selector::parse(".details > .byline > .comments_label > a").unwrap())
                    .next()
                    .unwrap()
                    .value()
                    .attr("href")
                    .unwrap()
                    .to_owned();
                format!("{URL}/{url}")
            },
            url: {
                let url = html
                    .select(&Selector::parse(".details > .link > a").unwrap())
                    .next()
                    .unwrap()
                    .value()
                    .attr("href")
                    .unwrap()
                    .to_owned();
                if description {
                    format!("{URL}/{url}")
                } else {
                    url
                }
            },
        }
    }

    pub(crate) fn url(&self) -> &String {
        &self.url
    }

    pub(crate) fn load_comments(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // In case the comments have already been loaded, just return.
        if self.comments_number == self.comments.len() {
            return Ok(());
        }
        let html = get_page(self.comments_url.clone())?;
        let comments_selector =
            Selector::parse("#inside > ol.comments > li.comments_subtree").unwrap();
        let comments_list = html.select(&comments_selector).skip(1);
        let comments = comments_list.map(Comment::from_html).collect();
        self.comments = comments;
        Ok(())
    }

    pub(crate) fn comments(&self) -> &Vec<Comment> {
        &self.comments
    }

    pub(crate) fn comments_descendants(&self) -> usize {
        self.comments()
            .iter()
            .map(|comment| comment.descendants_count())
            .sum()
    }

    pub(crate) fn comments_number(&self) -> usize {
        self.comments_number
    }
}

fn wrap_escaped_to_lines(s: &str, max_width: u16) -> Vec<Vec<String>> {
    let text_len = |s| console::strip_ansi_codes(s).len() as u16;
    let words = s.split_whitespace();
    let mut width = 0;
    let mut lines: Vec<Vec<String>> = vec![vec![]];
    for word in words {
        width += text_len(word) + 1;
        if width < max_width + 1 {
            // Stil fits within the line.
            lines.last_mut().unwrap().push(word.to_string());
        } else {
            // Word will not fit anymore.
            lines.push(vec![word.to_string()]);
            width = text_len(word) + 1;
        }
    }

    lines
}

fn wrap_with_indent(s: &str, max_width: u16, indent: u16) -> String {
    let spacer = " ".repeat(indent as usize);
    wrap_escaped_to_lines(s, max_width - indent)
        .iter()
        .map(|l| l.join(" "))
        .collect::<Vec<String>>()
        .join(&format!("\n{spacer}"))
}

pub(crate) fn display_story(story: &Story, columns: u16, selected: bool) -> String {
    //  26    The Windows malloc() Implementation Is A Trash Fire [c] [c++] [rant] erikmcclure.com
    //        via cadey 24 hours ago | 7 comments
    let tags = story
        .tags
        .iter()
        .map(|t| format!("{}", style(t.to_string()).color256(94)))
        .collect::<Vec<String>>()
        .join(" ");
    let upper = format!(
        "{title} {description} {tags}  {domain}",
        title = style(&story.title).bold(),
        description = if story.description { "☶ " } else { "" },
        domain = style(match &story.domain {
            Some(d) => d,
            None => "",
        })
        .italic()
        .dim()
    );
    let upper = wrap_with_indent(&upper, columns, 3 + 2);
    let lower = format!(
        "{} {} | {} comment{}",
        story.byline,
        story.time,
        story.comments_number,
        if story.comments_number == 1 { "" } else { "s" }
    );
    let lower = style(wrap_with_indent(&lower, columns, 3 + 2)).dim();
    let votes = if selected {
        style(story.votes).reverse().to_string()
    } else {
        story.votes.to_string()
    };
    format!(
        "{}{upper}\n{:>3}  {lower}",
        console::pad_str(&votes, 5, console::Alignment::Center, None),
        " "
    )
}
