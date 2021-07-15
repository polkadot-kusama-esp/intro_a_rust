use std::fmt;

trait Editable<T> {
    fn edit(&mut self, new_content: T);
}

trait LikeIncrementer<T: Default> {
    fn more_likes(&mut self, n: T);
}

trait Actions<T, U>: Editable<T> + LikeIncrementer<U>
where
    U: Default,
{
    fn random_likes(&mut self) {
        self.more_likes(Default::default())
    }
}

#[derive(Debug)]
struct Tweet {
    user_name: String,
    content: String,
    likes: u64,
    replies: Vec<Reply>,
}

impl<T> Editable<T> for Tweet
where
    T: Into<String> + fmt::Display,
{
    fn edit(&mut self, new_content: T) {
        self.content = new_content.into();
    }
}

impl fmt::Display for Tweet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}]: {}, likes: {}, replies: {}",
            self.user_name,
            self.content,
            self.likes,
            self.replies.len()
        )
    }
}

#[derive(Debug)]
struct Reply {
    user_name: String,
    content: String,
}

#[derive(Debug)]
struct Article {
    headline: String,
    location: String,
    author: String,
    content: String,
    category: Option<Category>,
}

impl<T: fmt::Display> Editable<T> for Article {
    fn edit(&mut self, new_content: T) {
        self.content = format!("{} UPDATED!", new_content);
    }
}

#[derive(Debug)]
enum Category {
    Politics,
    Sports,
    Economy,
    Health,
}

impl Default for Category {
    fn default() -> Self {
        Category::Economy
    }
}

fn main() {
    let replies = Vec::new();
    //replies = replies.insert(R);

    let mut tweet = Tweet {
        user_name: "hectorb".to_string(),
        content: "Check this new tweet".into(),
        likes: 0,
        replies,
    };

    tweet.edit("Check this updated tweet");

    println!("The tweet > {}", tweet);

    let mut article = Article {
        headline: "Surprising".into(),
        location: "Moon".into(),
        author: "olanod".into(),
        content: "New online event".into(),
        category: Some(Category::default()),
    };

    article.edit("New content for the article");

    println!("The article > {:?}", article);
}
