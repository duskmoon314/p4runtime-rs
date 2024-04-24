// use p4runtime::p4::config::v1 as p4cfgv1;
use p4runtime::p4::v1 as p4v1;
use tonic::codegen::*;

use crate::Client;

pub struct TableHelper<T, U>
where
    T: AsRef<Client<U>>,
{
    client: T,
    _phantom: std::marker::PhantomData<U>,
}

impl<T, U> TableHelper<T, U>
where
    T: AsRef<Client<U>>,
{
    pub fn new(client: T) -> Self {
        Self {
            client,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T, U> TableHelper<T, U>
where
    T: AsRef<Client<U>>,
    U: tonic::client::GrpcService<tonic::body::BoxBody>,
    U::Error: Into<StdError>,
    U::ResponseBody: Body<Data = Bytes> + Send + 'static,
    <U::ResponseBody as Body>::Error: Into<StdError> + Send,
{
    pub fn new_action(&self, action_name: &str, params: Vec<Vec<u8>>) -> p4v1::Action {
        let action_id = self.client.as_ref().p4info().action_id(action_name);
        let params = params
            .iter()
            .enumerate()
            .map(|(i, p)| p4v1::action::Param {
                param_id: (i + 1) as u32,
                value: p.clone(),
            })
            .collect();

        p4v1::Action { action_id, params }
    }

    pub fn new_table_action(&self, action_name: &str, params: Vec<Vec<u8>>) -> p4v1::TableAction {
        p4v1::TableAction {
            r#type: Some(p4v1::table_action::Type::Action(
                self.new_action(action_name, params),
            )),
        }
    }

    pub fn new_table_entry(
        &self,
        table_name: &str,
        match_fields: Vec<(String, p4v1::field_match::FieldMatchType)>,
        action: Option<p4v1::TableAction>,
        priority: i32,
    ) -> p4v1::TableEntry {
        let table_id = self.client.as_ref().p4info().table_id(table_name);
        let match_fields = match_fields
            .iter()
            .map(|(name, match_type)| p4v1::FieldMatch {
                field_id: self
                    .client
                    .as_ref()
                    .p4info()
                    .table_match_field_id(table_name, &name),
                field_match_type: Some(match_type.clone()),
            })
            .collect();

        p4v1::TableEntry {
            table_id,
            r#match: match_fields,
            action,
            priority,

            ..Default::default()
        }
    }
}

impl<T, U> TableHelper<T, U>
where
    T: AsRef<Client<U>> + AsMut<Client<U>>,
    U: tonic::client::GrpcService<tonic::body::BoxBody>,
    U::Error: Into<StdError>,
    U::ResponseBody: Body<Data = Bytes> + Send + 'static,
    <U::ResponseBody as Body>::Error: Into<StdError> + Send,
{
    pub async fn read_entry(
        &mut self,
        table_name: &str,
        match_fields: Vec<(String, p4v1::field_match::FieldMatchType)>,
    ) -> Result<p4v1::TableEntry, crate::error::ReadEntitySingleError> {
        let table_entry = self.new_table_entry(table_name, match_fields, None, 0);

        let entity = p4v1::Entity {
            entity: Some(p4v1::entity::Entity::TableEntry(table_entry)),
        };

        let entity = self.client.as_mut().read_entity_single(entity).await?;
        if let p4v1::entity::Entity::TableEntry(table_entry) = entity.entity.as_ref().unwrap() {
            Ok(table_entry.clone())
        } else {
            Err(crate::error::ReadEntitySingleError::UnexpectedEntity(
                "TableEntry".to_string(),
                format!("{:?}", entity),
            ))
        }
    }

    pub async fn read_entries(&mut self, table_name: &str) -> Result<Vec<p4v1::TableEntry>, ()> {
        let table_entry = self.new_table_entry(table_name, vec![], None, 0);

        let entity = p4v1::Entity {
            entity: Some(p4v1::entity::Entity::TableEntry(table_entry)),
        };

        let res = self
            .client
            .as_mut()
            .read_entity_wildcard(entity)
            .await
            .unwrap();

        let entries = res.iter().map(|e| {
            if let p4v1::entity::Entity::TableEntry(table_entry) = e.entity.as_ref().unwrap() {
                Ok(table_entry.clone())
            } else {
                Err(())
            }
        });

        entries.collect()
    }

    pub async fn insert_entry(
        &mut self,
        table_entry: p4v1::TableEntry,
    ) -> Result<tonic::Response<p4v1::WriteResponse>, tonic::Status> {
        let update = p4v1::Update {
            r#type: p4v1::update::Type::Insert as i32,
            entity: Some(p4v1::Entity {
                entity: Some(p4v1::entity::Entity::TableEntry(table_entry)),
            }),
        };

        self.client.as_mut().write_update(update).await
    }

    pub async fn modify_entry(
        &mut self,
        table_entry: p4v1::TableEntry,
    ) -> Result<tonic::Response<p4v1::WriteResponse>, tonic::Status> {
        let update = p4v1::Update {
            r#type: p4v1::update::Type::Modify as i32,
            entity: Some(p4v1::Entity {
                entity: Some(p4v1::entity::Entity::TableEntry(table_entry)),
            }),
        };

        self.client.as_mut().write_update(update).await
    }

    pub async fn delete_entry(
        &mut self,
        table_entry: p4v1::TableEntry,
    ) -> Result<tonic::Response<p4v1::WriteResponse>, tonic::Status> {
        let update = p4v1::Update {
            r#type: p4v1::update::Type::Delete as i32,
            entity: Some(p4v1::Entity {
                entity: Some(p4v1::entity::Entity::TableEntry(table_entry)),
            }),
        };

        self.client.as_mut().write_update(update).await
    }
}
