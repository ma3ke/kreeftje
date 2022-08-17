use crate::story::{display_story, prepend_string, Story};
use crate::{get_stories, STORIES_PER_SITE_PAGE};

#[derive(Clone, Copy)]
pub(crate) enum ViewMode {
    List,
    Comments,
}

pub(crate) struct View {
    stories: Vec<Story>,
    list_pos: usize,
    pub comments_pos: usize,
    page_size: usize,
    mode: ViewMode,
}

impl View {
    /// Creates a new empty view.
    pub(crate) fn new(page_size: usize) -> Self {
        Self {
            stories: Vec::new(),
            list_pos: 0,
            comments_pos: 0,
            page_size,
            mode: ViewMode::List,
        }
    }

    /// Load the next page of stories according to the number of stories stored.
    fn load_stories(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // The page numbers of the site are 1-indexed.
        let stories = &mut get_stories((self.stories.len() / STORIES_PER_SITE_PAGE) as u16 + 1)?;
        self.stories.append(stories);
        Ok(())
    }

    /// Get new pages of stories until the number of stories stored exceeds `pos`.
    fn load_stories_including(&mut self, pos: usize) -> Result<(), Box<dyn std::error::Error>> {
        while self.stories.len() <= pos {
            self.load_stories()?;
        }

        Ok(())
    }

    /// Get a new page of stories when pos steps onto the next presentation page and the current
    /// list of stories does not extend far enough to fill that page.
    pub(crate) fn load_stories_next_page(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // If the current stories list can accomodate another view page, do nothing. Otherwise,
        // load new stories.

        if self.stories.len() < self.list_pos + self.page_size - (self.list_pos % self.page_size) {
            self.load_stories_including(self.list_pos + STORIES_PER_SITE_PAGE - 1)?;
        }

        Ok(())
    }

    fn paginate(&mut self) -> Vec<(bool, Story)> {
        let mut chunks = self.stories.chunks(self.page_size);
        let chunk = chunks.nth(self.view_page()).unwrap();
        chunk
            .iter()
            .enumerate()
            .map(|(idx, story)| (idx == self.list_pos % self.page_size, story.to_owned()))
            .collect()
    }

    pub(crate) fn go_to(&mut self, travel: Travel) {
        match self.mode {
            ViewMode::List => {
                match travel {
                    Travel::NextStep => self.list_pos += self.page_size,
                    Travel::PrevStep => self.list_pos -= self.list_pos.min(self.page_size),
                    Travel::NextItem => self.list_pos += 1,
                    Travel::PrevItem => self.list_pos -= self.list_pos.min(1),
                    Travel::Top => self.list_pos = 0,
                    Travel::Bottom => self.list_pos = self.stories.len() - 1,
                }
                // Reset the comments_pos every time a movement occurs in the List ViewMode.
                self.comments_pos = 0;
            }
            ViewMode::Comments => match travel {
                Travel::NextStep => self.list_pos += 1,
                Travel::PrevStep => self.list_pos -= self.list_pos.min(1),
                Travel::NextItem => self.comments_pos += 1,
                Travel::PrevItem => self.comments_pos -= self.comments_pos.min(1),
                Travel::Top => self.comments_pos = 0,
                // HACK: This is a very lousy way of traveling to the bottom of the comments view.
                Travel::Bottom => self.comments_pos = 1_000_000,
            },
        }
    }

    fn get_story(&self, pos: usize) -> Option<&Story> {
        self.stories.get(pos)
    }

    fn get_story_mut(&mut self, pos: usize) -> Option<&mut Story> {
        self.stories.get_mut(pos)
    }

    pub(crate) fn get_selected_story(&self) -> &Story {
        self.get_story(self.list_pos).unwrap()
    }

    pub(crate) fn get_selected_story_mut(&mut self) -> &mut Story {
        self.get_story_mut(self.list_pos).unwrap()
    }

    pub(crate) fn generate_string(&mut self, width: u16, height: u16) -> String {
        match self.mode {
            ViewMode::List => {
                let current_stories_page = self.paginate();
                let displayed_stories = current_stories_page
                    .into_iter()
                    .map(|(selected, story)| display_story(&story, width - 3, selected));
                displayed_stories.collect::<Vec<String>>().join("\n")
            }
            ViewMode::Comments => {
                let margin = 2;
                let comments = self.get_selected_story().comments();
                if !comments.is_empty() {
                    let lines = comments
                        .iter()
                        .map(|comment| {
                            prepend_string(
                                &comment.to_string(width as usize - (margin * 2)),
                                &" ".repeat(margin),
                            )
                        })
                        .collect::<Vec<String>>()
                        .join("\n")
                        .lines()
                        .map(|line| line.to_string())
                        .collect::<Vec<String>>();

                    eprintln!("len: {}, height: {height}", lines.len());
                    if lines.len() > height as usize {
                        self.comments_pos =
                            self.comments_pos.clamp(0, lines.len() - height as usize);
                        let start = self.comments_pos.clamp(0, lines.len() - height as usize);
                        let end = lines.len().min(self.comments_pos + height as usize);
                        lines[start..end].join("\n")
                    } else {
                        lines.join("\n")
                    }
                } else {
                    "No comments, yet.".to_string()
                }
            }
        }
    }

    pub(crate) fn pos(&self) -> usize {
        self.list_pos
    }

    fn view_page(&self) -> usize {
        self.list_pos / self.page_size
    }

    /// Returns the number of the site page the story under the position of the view can be found
    /// on.
    ///
    /// ## Note
    ///
    /// The site page number is 1-indexed.
    pub(crate) fn site_page(&self) -> usize {
        self.list_pos / STORIES_PER_SITE_PAGE + 1
    }

    pub(crate) fn view_list(&mut self) {
        self.mode = ViewMode::List
    }

    pub(crate) fn view_comments(&mut self) {
        self.mode = ViewMode::Comments
    }

    pub(crate) fn view_toggle(&mut self) {
        self.mode = match self.mode {
            ViewMode::List => ViewMode::Comments,
            ViewMode::Comments => ViewMode::List,
        }
    }

    pub(crate) fn mode(&self) -> ViewMode {
        self.mode
    }
}

pub(crate) enum Travel {
    NextStep,
    PrevStep,
    NextItem,
    PrevItem,
    Top,
    Bottom,
}
