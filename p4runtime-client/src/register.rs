use p4runtime::p4::v1 as p4v1;
use tonic::codegen::*;

use crate::Client;

pub struct RegisterHelper<T, U>
where
    T: AsRef<Client<U>>,
{
    client: T,
    _phantom: std::marker::PhantomData<U>,
}

impl<T, U> RegisterHelper<T, U>
where
    T: AsRef<Client<U>>,
{
    pub fn new(client: T) -> Self {
        RegisterHelper {
            client,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn new_entry(
        &self,
        register_name: &str,
        index: Option<i64>,
        data: Option<p4v1::P4Data>,
    ) -> p4v1::RegisterEntry {
        let register_id = self.client.as_ref().p4info().register_id(register_name);

        p4v1::RegisterEntry {
            register_id,
            index: index.map(Into::into),
            data,
        }
    }
}

impl<T, U> RegisterHelper<T, U>
where
    T: AsRef<Client<U>> + AsMut<Client<U>>,
    U: tonic::client::GrpcService<tonic::body::BoxBody>,
    U::Error: Into<StdError>,
    U::ResponseBody: Body<Data = Bytes> + Send + 'static,
    <U::ResponseBody as Body>::Error: Into<StdError> + Send,
{
    pub async fn read_entry(
        &mut self,
        register_name: &str,
        index: i64,
    ) -> Result<p4v1::RegisterEntry, crate::error::ReadEntitySingleError> {
        let register_entry = self.new_entry(register_name, Some(index), None);

        let entity = p4v1::Entity {
            entity: Some(p4v1::entity::Entity::RegisterEntry(register_entry)),
        };

        let entity: p4v1::Entity = self.client.as_mut().read_entity_single(entity).await?;
        if let p4v1::entity::Entity::RegisterEntry(register_entry) = entity.entity.as_ref().unwrap()
        {
            Ok(register_entry.clone())
        } else {
            Err(crate::error::ReadEntitySingleError::UnexpectedEntity(
                "RegisterEntry".to_string(),
                format!("{:?}", entity),
            ))
        }
    }
}
