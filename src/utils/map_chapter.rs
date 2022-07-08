use anyhow::Result;

use mdbook::book::Book;
use mdbook::book::BookItem;
use mdbook::book::Chapter;

pub fn map_chapter<F>(book: &mut Book, func: &mut F) -> Result<()>
where
    F: FnMut(&mut Chapter) -> Result<()>,
{
    fn _map_chapter_on<F>(item: &mut BookItem, func: &mut F) -> Result<()>
    where
        F: FnMut(&mut Chapter) -> Result<()>,
    {
        match item {
            BookItem::Chapter(ref mut chapter) => {
                func(chapter)?;

                for sub_item in &mut chapter.sub_items {
                    _map_chapter_on(sub_item, func)?;
                }
            }
            BookItem::PartTitle(_) | BookItem::Separator => {}
        }
        Ok(())
    }

    for item in &mut book.sections {
        _map_chapter_on(item, func)?;
    }

    Ok(())
}
