use leptos::prelude::*;
use crate::models::extra_item::{ExtraItem, ExtraItemForm};

#[cfg(feature = "ssr")]
use leptos::prelude::ServerFnError;


#[server]
pub async fn insert_extra_item(
    extra_item_form: ExtraItemForm,
) -> Result<ExtraItem, ServerFnError> {
    use crate::api::ssr::*;
    let db = &mut get_db()?;
    server_err!(
        extra_item_form.insert(db),
        "Could not insert extra_item {:?}",
        extra_item_form
    )
}

#[server]
pub async fn delete_extra_item(
    id: i32,
) -> Result<usize, ServerFnError> {
    use crate::api::ssr::*;

    let db = &mut get_db()?;
    server_err!(
        ExtraItem::delete(db, id),
        "Could not delete extra_item with id {id}",
    )
}

#[server]
pub async fn get_extra_items(
) -> Result<Vec<ExtraItem>, ServerFnError> {
    use crate::api::ssr::*;

    let db = &mut get_db()?;
    server_err!(
        ExtraItem::get_all(db),
        "Could not get extra items"
    )
}

#[server]
pub async fn update_extra_item(extra_item: ExtraItem) -> Result<ExtraItem, ServerFnError> {
    use crate::api::ssr::*;
    let db = &mut get_db()?;
    server_err!(
        extra_item.update(db),
        "Could not update extra item {extra_item:?}"
    )
}

#[server]
pub async fn get_extra_item(id: i32) -> Result<ExtraItem, ServerFnError> {
    use crate::api::ssr::*;
    let db = &mut get_db()?;

    server_err!(
        ExtraItem::get(db, id),
        "Could not get meal with id {id}"
    )
}

#[cfg(feature = "ssr")]
#[cfg(test)]
mod test {
    use super::*;
    use crate::db::tests::TEST_POOL;
    use diesel::Connection;

    #[test]
    pub fn test_extra_items_all() {
        let db = &mut TEST_POOL.clone().get().unwrap();
        db.test_transaction(|db| -> Result<(), ()> {
            let mut extra_item = ExtraItemForm{
                amount: 0,
                bought: false,
                name: String::new()
            }.insert(db).unwrap();

            let extra_item_other = ExtraItemForm{
                amount: 0,
                bought: false,
                name: String::new()
            }.insert(db).unwrap();

            assert_eq!(ExtraItem::get_all(db).unwrap().len(), 2);
            assert_eq!(extra_item, ExtraItem::get(db, extra_item.id).unwrap());
            extra_item.bought = true;
            let extra_item = extra_item.update(db).unwrap();
            assert_ne!(ExtraItem::get(db, extra_item.id).unwrap().bought, ExtraItem::get(db, extra_item_other.id).unwrap().bought);
            ExtraItem::delete(db, extra_item_other.id).unwrap();
            assert_eq!(ExtraItem::get_all(db).unwrap().len(), 1);
            assert_eq!(ExtraItem::get_all(db).unwrap(), vec![extra_item]);

            Ok(())
        });
    }
}