use p4runtime::p4::v1::{self as p4v1, p4_runtime_client::P4RuntimeClient, CapabilitiesRequest};
use p4runtime_client::{client::ClientOptions, Client};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let p4bin = include_bytes!("../build/main.json");
    let p4info = include_bytes!("../build/main.p4info.bin");

    let mut p4rt_client = P4RuntimeClient::connect("http://127.0.0.1:9559").await?;

    let res = p4rt_client.capabilities(CapabilitiesRequest {}).await?;
    println!(
        "Server P4Runtime capabilities: {:?}",
        res.get_ref().p4runtime_api_version
    );

    let mut client = Client::new(
        p4rt_client,
        0,    // device_id
        1,    // election_id
        None, // role
        ClientOptions::default(),
    );

    println!("Starting arbitration...");
    client.arbitration().await?;

    client.p4info_mut().load_bytes(p4info)?;

    println!("Setting pipeline config...");
    client.set_forwarding_pipeline(p4bin.to_vec()).await?;

    let entry1 = client.table().new_table_entry(
        "ipv4_lpm",
        vec![(
            "hdr.ipv4.dst".to_string(),
            p4v1::field_match::FieldMatchType::Lpm(p4v1::field_match::Lpm::new(
                vec![10, 0, 1, 1],
                32,
                true,
            )),
        )],
        Some(
            client
                .table()
                .new_table_action("ipv4_forward", vec![vec![8, 0, 0, 0, 1, 0x11], vec![1]]),
        ),
        0,
    );
    let entry2 = client.table().new_table_entry(
        "ipv4_lpm",
        vec![(
            "hdr.ipv4.dst".to_string(),
            p4v1::field_match::FieldMatchType::Lpm(p4v1::field_match::Lpm::new(
                vec![10, 0, 2, 1],
                32,
                true,
            )),
        )],
        Some(
            client
                .table()
                .new_table_action("ipv4_forward", vec![vec![8, 0, 0, 0, 2, 0x22], vec![2]]),
        ),
        0,
    );
    client.table_mut().insert_entry(entry1).await?;
    client.table_mut().insert_entry(entry2).await?;

    println!("Table entries inserted. Try h1 ping h2 now.");

    Ok(())
}
