use crate::DResult;
use crate::db::DBPool;
use crate::models::item::Item;
use itertools::Itertools;
use std::collections::{BTreeSet, BTreeMap};
use log::info;

mod room;

pub struct StaticItem {
    pub item_code: &'static str,
    pub initial_item: Box<dyn Fn() -> Item>
}

struct StaticItemTypeGroup {
    item_type: &'static str,
    items: fn () -> Box<dyn Iterator<Item = StaticItem>>
}

fn static_item_registry() -> Vec<StaticItemTypeGroup> {
    vec!(
        // Must have no duplicates.
        StaticItemTypeGroup {
            item_type: "npc",
            items: || Box::new(vec!().into_iter())
        },
        StaticItemTypeGroup {
            item_type: "room",
            items: || room::room_static_items()
        },
    )
}
    

async fn refresh_static_items(pool: &DBPool) -> DResult<()> {
    let registry = static_item_registry();
    
    let expected_type: BTreeSet<String> =
        registry.iter().map(|x| x.item_type.to_owned()).collect();
    let cur_types: Box<BTreeSet<String>> = pool.find_static_item_types().await?;
    for item_type in cur_types.difference(&expected_type) {
        pool.delete_static_items_by_type(item_type).await?;
    }
    
    for type_group in registry.iter() {
        info!("Checking static_content of item_type {}", type_group.item_type);
        let tx = pool.start_transaction().await?;
        let existing_items = tx.find_static_items_by_type(type_group.item_type).await?;
        let expected_items: BTreeMap<String, StaticItem> =
            (type_group.items)().map(|x| (x.item_code.to_owned(), x)).collect();
        let expected_set: BTreeSet<String> = expected_items.keys().map(|x|x.to_owned()).collect();
        for unwanted_item in existing_items.difference(&expected_set) {
            tx.delete_static_items_by_code(type_group.item_type, unwanted_item).await?;
        }
        for new_item_code in expected_set.difference(&existing_items) {
            tx.create_item(&(expected_items.get(new_item_code)
                             .unwrap().initial_item)()).await?;
        }
        tx.commit().await?;
        info!("Committed any changes for static_content of item_type {}", type_group.item_type);
    }
    Ok(())
}

pub async fn refresh_static_content(pool: &DBPool) -> DResult<()> {
    refresh_static_items(pool).await?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn no_duplicate_static_content() {
            let mut registry = static_item_registry();
        registry.sort_unstable_by(|x, y| x.item_type.cmp(y.item_type));
    
        let duplicates: Vec<&'static str> =
            registry.iter()
            .group_by(|x| x.item_type).into_iter()
            .filter_map(|(k, v)| if v.count() <= 1 { None } else { Some(k) })
            .collect();
        if duplicates.len() > 0 {
            panic!("static_item_registry has duplicate item_types: {:}", duplicates.join(", "));
        }

        for type_group in registry.iter() {
            let iterator : Box<dyn Iterator<Item = StaticItem>> = (type_group.items)();
            let duplicates: Vec<&'static str> = iterator
                .group_by(|x| x.item_code)
                .into_iter()
                .filter_map(|(k, v)| if v.count() <= 1 { None } else { Some(k) })
                .collect();
            if duplicates.len() > 0 {
                panic!("static_item_registry has duplicate item_codes for {}: {:}",
                       type_group.item_type,
                       duplicates.join(", "));
            }
        }
    }
}
