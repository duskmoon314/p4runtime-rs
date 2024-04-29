use p4runtime::p4::v1 as p4v1;
use tonic::codegen::*;

use crate::{error::ReadEntitiesError, Client};

pub struct CounterHelper<T, U>
where
    T: AsRef<Client<U>>,
{
    client: T,
    _phantom: std::marker::PhantomData<U>,
}

impl<T, U> CounterHelper<T, U>
where
    T: AsRef<Client<U>>,
{
    pub fn new(client: T) -> Self {
        CounterHelper {
            client,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn new_entry(
        &self,
        counter_name: &str,
        index: Option<i64>,
        data: Option<p4v1::CounterData>,
    ) -> p4v1::CounterEntry {
        let counter_id = self.client.as_ref().p4info().counter_id(counter_name);

        p4v1::CounterEntry {
            counter_id,
            index: index.map(Into::into),
            data,
        }
    }
}

impl<T, U> CounterHelper<T, U>
where
    T: AsRef<Client<U>> + AsMut<Client<U>>,
    U: tonic::client::GrpcService<tonic::body::BoxBody>,
    U::Error: Into<StdError>,
    U::ResponseBody: Body<Data = Bytes> + Send + 'static,
    <U::ResponseBody as Body>::Error: Into<StdError> + Send,
{
    pub async fn read_entry(
        &mut self,
        counter_name: &str,
        index: i64,
    ) -> Result<p4v1::CounterEntry, crate::error::ReadEntitySingleError> {
        let counter_entry = self.new_entry(counter_name, Some(index), None);

        let entity = p4v1::Entity {
            entity: Some(p4v1::entity::Entity::CounterEntry(counter_entry)),
        };

        let entity: p4v1::Entity = self.client.as_mut().read_entity_single(entity).await?;
        if let p4v1::entity::Entity::CounterEntry(counter_entry) = entity.entity.as_ref().unwrap() {
            Ok(counter_entry.clone())
        } else {
            Err(crate::error::ReadEntitySingleError::UnexpectedEntity(
                "CounterEntry".to_string(),
                format!("{:?}", entity),
            ))
        }
    }

    /// Read multiple entries using wildcard
    ///
    /// If `counter_name` is `None`, all counters will be read.
    /// If `index` is `None`, all entries will be read.
    pub async fn read_entries(
        &mut self,
        counter_name: Option<&str>,
        index: Option<i64>,
    ) -> Result<Vec<p4v1::CounterEntry>, ReadEntitiesError> {
        let entry = if let Some(counter_name) = counter_name {
            self.new_entry(counter_name, index, None)
        } else {
            p4v1::CounterEntry {
                counter_id: 0,
                index: index.map(Into::into),
                data: None,
            }
        };

        let entity = p4v1::Entity {
            entity: Some(p4v1::entity::Entity::CounterEntry(entry)),
        };

        let entities: Vec<p4v1::Entity> = self.client.as_mut().read_entity_wildcard(entity).await?;

        let entries = entities
            .into_iter()
            .map(|e| {
                if let p4v1::entity::Entity::CounterEntry(entry) = e.entity.as_ref().unwrap() {
                    Ok(entry.clone())
                } else {
                    Err(ReadEntitiesError::UnexpectedEntity(
                        "CounterEntry".to_string(),
                        format!("{:?}", e),
                    ))
                }
            })
            .collect::<Result<Vec<p4v1::CounterEntry>, ReadEntitiesError>>()?;

        Ok(entries)
    }

    pub async fn modify_entry(
        &mut self,
        counter_name: &str,
        index: i64,
        data: p4v1::CounterData,
    ) -> Result<p4v1::WriteResponse, tonic::Status> {
        let counter_entry = self.new_entry(counter_name, Some(index), Some(data));

        let update = p4v1::Update {
            r#type: p4v1::update::Type::Modify as i32,
            entity: Some(p4v1::Entity {
                entity: Some(p4v1::entity::Entity::CounterEntry(counter_entry)),
            }),
        };

        let res = self.client.as_mut().write_update(update).await?;
        Ok(res.into_inner())
    }
}
