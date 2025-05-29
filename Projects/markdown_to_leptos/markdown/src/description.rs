use chrono::{DateTime, Utc};
use quote::{ToTokens, quote};

#[derive(Default)]
pub struct Description {
    pub title: String,
    pub date: DateTime<Utc>,
    pub tags: String,
    pub path: String,
}

impl ToTokens for Description {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let title = &self.title;
        let path = &self.path;
        let date = &self.date.to_string();
        let tags = &self.tags;
        tokens.extend(quote! {
            <A href=#path>
                <span class="title">#title</span>
                <span class="date">#date</span>
                <span class="tags">#tags</span>
            </A>
        })
    }
}

impl Description {
    pub fn new(path: String) -> Self {
        Self {
            title: Default::default(),
            date: Default::default(),
            tags: Default::default(),
            path,
        }
    }

    pub fn reverse_as_date(descriptions: &mut [Description]) {
        descriptions.sort_by_key(|description| std::cmp::Reverse(description.date));
    }
}
