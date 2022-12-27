use super::StaticItem;

pub fn static_items() -> Box<dyn Iterator<Item = StaticItem>> {
    Box::new(vec!().into_iter())
}
