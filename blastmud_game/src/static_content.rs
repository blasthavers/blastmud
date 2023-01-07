use crate::DResult;
use crate::db::DBPool;
use crate::models::{item::Item, task::Task};
use std::collections::{BTreeSet, BTreeMap};
use log::info;

pub mod room;
pub mod npc;
mod fixed_item;

pub struct StaticItem {
    pub item_code: &'static str,
    pub initial_item: Box<dyn Fn() -> Item>
}

pub struct StaticTask {
    pub task_code: String,
    pub initial_task: Box<dyn Fn() -> Task>
}

struct StaticThingTypeGroup<Thing> {
    thing_type: &'static str,
    things: fn () -> Box<dyn Iterator<Item = Thing>>
}

fn static_item_registry() -> Vec<StaticThingTypeGroup<StaticItem>> {
    vec!(
        // Must have no duplicates.
        StaticThingTypeGroup::<StaticItem> {
            thing_type: "npc",
            things: || npc::npc_static_items()
        },
        StaticThingTypeGroup::<StaticItem> {
            thing_type: "room",
            things: || room::room_static_items()
        },
        StaticThingTypeGroup::<StaticItem> {
            thing_type: "fixed_item",
            things: || fixed_item::static_items()
        },
    )
}

fn static_task_registry() -> Vec<StaticThingTypeGroup<StaticTask>> {
    vec!(
        // Must have no duplicates.
        StaticThingTypeGroup::<StaticTask> {
            thing_type: "NPCSay",
            things: || npc::npc_say_tasks()
        },
    )
}

async fn refresh_static_items(pool: &DBPool) -> DResult<()> {
    let registry = static_item_registry();
    
    let expected_type: BTreeSet<String> =
        registry.iter().map(|x| x.thing_type.to_owned()).collect();
    let cur_types: Box<BTreeSet<String>> = pool.find_static_item_types().await?;
    for item_type in cur_types.difference(&expected_type) {
        pool.delete_static_items_by_type(item_type).await?;
    }
    
    for type_group in registry.iter() {
        info!("Checking static_content of item_type {}", type_group.thing_type);
        let tx = pool.start_transaction().await?;
        let existing_items = tx.find_static_items_by_type(type_group.thing_type).await?;
        let expected_items: BTreeMap<String, StaticItem> =
            (type_group.things)().map(|x| (x.item_code.to_owned(), x)).collect();
        let expected_set: BTreeSet<String> = expected_items.keys().map(|x|x.to_owned()).collect();
        for unwanted_item in existing_items.difference(&expected_set) {
            info!("Deleting item {:?}", unwanted_item);
            tx.delete_static_items_by_code(type_group.thing_type, unwanted_item).await?;
        }
        for new_item_code in expected_set.difference(&existing_items) {
            info!("Creating item {:?}", new_item_code);
            tx.create_item(&(expected_items.get(new_item_code)
                             .unwrap().initial_item)()).await?;
        }
        for existing_item_code in expected_set.intersection(&existing_items) {
            tx.limited_update_static_item(
                &(expected_items.get(existing_item_code)
                  .unwrap().initial_item)()).await?;
        }
        tx.commit().await?;
        info!("Committed any changes for static_content of item_type {}", type_group.thing_type);
    }
    Ok(())
}

async fn refresh_static_tasks(pool: &DBPool) -> DResult<()> {
    let registry = static_task_registry();
    
    let expected_type: BTreeSet<String> =
        registry.iter().map(|x| x.thing_type.to_owned()).collect();
    let cur_types: Box<BTreeSet<String>> = pool.find_static_task_types().await?;
    for task_type in cur_types.difference(&expected_type) {
        pool.delete_static_tasks_by_type(task_type).await?;
    }
    
    for type_group in registry.iter() {
        info!("Checking static_content of task_type {}", type_group.thing_type);
        let tx = pool.start_transaction().await?;
        let existing_tasks = tx.find_static_tasks_by_type(type_group.thing_type).await?;
        let expected_tasks: BTreeMap<String, StaticTask> =
            (type_group.things)().map(|x| (x.task_code.to_owned(), x)).collect();
        let expected_set: BTreeSet<String> = expected_tasks.keys().map(|x|x.to_owned()).collect();
        for unwanted_task in existing_tasks.difference(&expected_set) {
            info!("Deleting task {:?}", unwanted_task);
            tx.delete_static_tasks_by_code(type_group.thing_type, unwanted_task).await?;
        }
        for new_task_code in expected_set.difference(&existing_tasks) {
            info!("Creating task {:?}", new_task_code);
            tx.upsert_task(&(expected_tasks.get(new_task_code)
                             .unwrap().initial_task)()).await?;
        }
        for existing_task_code in expected_set.intersection(&existing_tasks) {
            tx.limited_update_static_task(
                &(expected_tasks.get(existing_task_code)
                  .unwrap().initial_task)()).await?;
        }
        tx.commit().await?;
        info!("Committed any changes for static_content of task_type {}", type_group.thing_type);
    }
    Ok(())
}

pub async fn refresh_static_content(pool: &DBPool) -> DResult<()> {
    refresh_static_items(pool).await?;
    refresh_static_tasks(pool).await?;
    Ok(())
}

#[cfg(test)]
mod test {
    use itertools::Itertools;
    use super::*;

    #[test]
    fn no_duplicate_static_items() {
        let mut registry = static_item_registry();
        registry.sort_unstable_by(|x, y| x.thing_type.cmp(y.thing_type));
    
        let duplicates: Vec<&'static str> =
            registry.iter()
            .group_by(|x| x.thing_type).into_iter()
            .filter_map(|(k, v)| if v.count() <= 1 { None } else { Some(k) })
            .collect();
        if duplicates.len() > 0 {
            panic!("static_item_registry has duplicate item_types: {:}", duplicates.join(", "));
        }

        for type_group in registry.iter() {
            let iterator : Box<dyn Iterator<Item = StaticItem>> = (type_group.things)();
            let duplicates: Vec<&'static str> = iterator
                .group_by(|x| x.item_code)
                .into_iter()
                .filter_map(|(k, v)| if v.count() <= 1 { None } else { Some(k) })
                .collect();
            if duplicates.len() > 0 {
                panic!("static_item_registry has duplicate item_codes for {}: {:}",
                       type_group.thing_type,
                       duplicates.join(", "));
            }
        }
    }

    #[test]
    fn no_duplicate_static_tasks() {
        let mut registry = static_task_registry();
        registry.sort_unstable_by(|x, y| x.thing_type.cmp(y.thing_type));
    
        let duplicates: Vec<&'static str> =
            registry.iter()
            .group_by(|x| x.thing_type).into_iter()
            .filter_map(|(k, v)| if v.count() <= 1 { None } else { Some(k) })
            .collect();
        if duplicates.len() > 0 {
            panic!("static_task_registry has duplicate task_types: {:}", duplicates.join(", "));
        }

        for type_group in registry.iter() {
            let iterator : Box<dyn Iterator<Item = StaticTask>> = (type_group.things)();
            let duplicates: Vec<String> = iterator
                .group_by(|x| x.task_code.clone())
                .into_iter()
                .filter_map(|(k, v)| if v.count() <= 1 { None } else { Some(k) })
                .collect();
            if duplicates.len() > 0 {
                panic!("static_task_registry has duplicate task_codes for {}: {:}",
                       type_group.thing_type,
                       duplicates.join(", "));
            }
        }
    }
}
