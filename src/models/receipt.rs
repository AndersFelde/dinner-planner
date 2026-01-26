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
}
#[cfg(feature = "ssr")]
impl ReceiptWithItems {
    pub fn get(db: &mut DbConn, id: i32) -> Result<ReceiptWithItems, Error> {
        let receipt: Receipt = receipts::table.find(id).first(db)?;

        // Load all items belonging to this receipt
        let items: Vec<ReceiptItem> = ReceiptItem::belonging_to(&receipt).load(db)?;
        Ok(ReceiptWithItems { receipt, items })
    }

    pub fn get_all(db: &mut DbConn) -> Result<Vec<ReceiptWithItems>, Error> {
        let receipts: Vec<Receipt> = receipts::table.load(db)?;

        // Load all receipt items belonging to those receipts
        let items: Vec<ReceiptItem> = ReceiptItem::belonging_to(&receipts).load(db)?;

        // Group items by receipt
        let grouped_items = items.grouped_by(&receipts);

        // Zip receipts with their items
        let result = receipts
            .into_iter()
            .zip(grouped_items)
            .map(|(receipt, items)| ReceiptWithItems { receipt, items })
            .collect::<Vec<ReceiptWithItems>>();

        Ok(result)
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
