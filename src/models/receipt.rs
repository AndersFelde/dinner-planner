use chrono::NaiveDateTime;
#[cfg(feature = "ssr")]
use diesel::prelude::*;

#[cfg(feature = "ssr")]
use crate::api::ssr::*;

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct ReceiptWithItems {
    #[serde(flatten)]
    pub receipt: Receipt,
    pub items: Vec<ReceiptItem>,
    pub days: Option<Vec<crate::models::day::Day>>,
}
#[cfg(feature = "ssr")]
impl ReceiptWithItems {
    pub fn get(db: &mut DbConn, id: i32) -> Result<ReceiptWithItems, Error> {
        let receipt: Receipt = receipts::table.find(id).first(db)?;

        // Load all items belonging to this receipt
        let items: Vec<ReceiptItem> = ReceiptItem::belonging_to(&receipt).load(db)?;
        Ok(ReceiptWithItems {
            days: Day::get_by_receipt(db, receipt.id)?,
            receipt,
            items,
        })
    }

    pub fn get_all(db: &mut DbConn) -> Result<Vec<ReceiptWithItems>, Error> {
        let receipts: Vec<Receipt> = receipts::table.order_by(receipts::id.desc()).load(db)?;

        // Load all receipt items belonging to those receipts
        let items: Vec<ReceiptItem> = ReceiptItem::belonging_to(&receipts).load(db)?;

        // Group items by receipt
        let grouped_items = items.grouped_by(&receipts);

        // Zip receipts with their items
        let result = receipts
            .into_iter()
            .zip(grouped_items)
            .map(|(receipt, items)| {
                Ok(ReceiptWithItems {
                    days: Day::get_by_receipt(db, receipt.id)?,
                    receipt,
                    items,
                })
            })
            .collect::<Result<Vec<ReceiptWithItems>, Error>>()?;

        Ok(result)
    }

    pub fn get_by_day(
        db: &mut DbConn,
        day_id: i32,
    ) -> Result<Option<Vec<ReceiptWithItems>>, Error> {
        use crate::schema::receipt_days;

        let receipts_list: Vec<Receipt> = receipts::table
            .inner_join(receipt_days::table.inner_join(days::table))
            .filter(days::id.eq(day_id))
            .select(receipts::all_columns)
            .distinct()
            .load(db)?;

        // Load all items for these receipts
        let items: Vec<ReceiptItem> = ReceiptItem::belonging_to(&receipts_list).load(db)?;
        let grouped_items = items.grouped_by(&receipts_list);

        // Combine receipts with their items
        let result: Vec<ReceiptWithItems> = receipts_list
            .into_iter()
            .zip(grouped_items)
            .map(|(receipt, items)| {
                Ok(ReceiptWithItems {
                    days: Day::get_by_receipt(db, receipt.id)?,
                    receipt,
                    items,
                })
            })
            .collect::<Result<Vec<ReceiptWithItems>, Error>>()?;

        Ok((!result.is_empty()).then_some(result))
    }
}

impl ReceiptWithItems {
    pub fn total(&self) -> f32 {
        self.items.iter().map(|i| i.price).sum()
    }

    pub fn anders_sum(&self) -> f32 {
        self.items
            .iter()
            .filter(|i| i.anders_pay)
            .map(|i| i.price / (i.ac_pay as u8 + i.anders_pay as u8 + i.andreas_pay as u8) as f32)
            .sum()
    }

    pub fn andreas_sum(&self) -> f32 {
        self.items
            .iter()
            .filter(|i| i.andreas_pay)
            .map(|i| i.price / (i.ac_pay as u8 + i.anders_pay as u8 + i.andreas_pay as u8) as f32)
            .sum()
    }

    pub fn ac_sum(&self) -> f32 {
        self.items
            .iter()
            .filter(|i| i.ac_pay)
            .map(|i| i.price / (i.ac_pay as u8 + i.anders_pay as u8 + i.andreas_pay as u8) as f32)
            .sum()
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
#[cfg_attr(feature = "ssr", derive(Insertable, AsChangeset))]
#[cfg_attr(feature = "ssr", diesel(table_name = crate::schema::receipts))]
pub struct ReceiptForm {
    pub store: String,
    pub datetime: NaiveDateTime,
}

#[cfg(feature = "ssr")]
impl ReceiptForm {
    pub fn insert(&self, db: &mut DbConn) -> Result<Receipt, Error> {
        insert_into(receipts::table).values(self).get_result(db)
    }
}

#[cfg_attr(
    feature = "ssr",
    derive(Queryable, Selectable, Identifiable, PartialEq, AsChangeset)
)]
#[cfg_attr(feature = "ssr", diesel(table_name = crate::schema::receipts))]
#[cfg_attr(feature = "ssr", diesel(check_for_backend(diesel::sqlite::Sqlite)))]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct Receipt {
    pub id: i32,
    pub store: String,
    pub datetime: NaiveDateTime,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
#[cfg_attr(feature = "ssr", derive(Insertable, AsChangeset))]
#[cfg_attr(feature = "ssr", diesel(table_name = crate::schema::receipt_items))]
pub struct ReceiptItemForm {
    pub receipt_id: i32,
    pub name: String,
    pub price: f32,
    pub anders_pay: bool,
    pub andreas_pay: bool,
    pub ac_pay: bool,
}

#[cfg(feature = "ssr")]
impl ReceiptItemForm {
    pub fn insert(&self, db: &mut DbConn) -> Result<ReceiptItem, Error> {
        insert_into(receipt_items::table)
            .values(self)
            .get_result(db)
    }
}

#[cfg_attr(
    feature = "ssr",
    derive(
        Queryable,
        Selectable,
        Identifiable,
        PartialEq,
        AsChangeset,
        Associations
    )
)]
#[cfg_attr(feature = "ssr", diesel(belongs_to(crate::models::receipt::Receipt)))]
#[cfg_attr(feature = "ssr", diesel(table_name = crate::schema::receipt_items))]
#[cfg_attr(feature = "ssr", diesel(check_for_backend(diesel::sqlite::Sqlite)))]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct ReceiptItem {
    pub id: i32,
    pub receipt_id: i32,
    pub name: String,
    pub price: f32,
    pub anders_pay: bool,
    pub andreas_pay: bool,
    pub ac_pay: bool,
}

#[cfg_attr(
    feature = "ssr",
    derive(Identifiable, Insertable, Queryable, Selectable, Associations,)
)]
#[cfg_attr(feature = "ssr", diesel(belongs_to(Receipt)))]
#[cfg_attr(feature = "ssr", diesel(belongs_to(Day)))]
#[cfg_attr(feature = "ssr", diesel(table_name = crate::schema::receipt_days))]
#[cfg_attr(feature = "ssr", diesel(primary_key(day_id, receipt_id)))]
#[cfg_attr(feature = "ssr", diesel(check_for_backend(diesel::sqlite::Sqlite)))]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct ReceiptDay {
    pub day_id: i32,
    pub receipt_id: i32,
}

#[cfg(feature = "ssr")]
impl ReceiptDay {
    pub fn upsert(&self, db: &mut DbConn) -> Result<ReceiptDay, Error> {
        use crate::schema::receipt_days;

        insert_into(receipt_days::table)
            .values(self)
            .on_conflict((receipt_days::day_id, receipt_days::receipt_id))
            .do_nothing()
            .get_result::<ReceiptDay>(db)
    }
}
