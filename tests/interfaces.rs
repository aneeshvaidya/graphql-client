#[macro_use]
extern crate graphql_client;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

const RESPONSE: &'static str = include_str!("interfaces/interface_response.json");

#[derive(GraphQLQuery)]
#[GraphQLQuery(
    query_path = "tests/interfaces/interface_query.graphql",
    schema_path = "tests/interfaces/interface_schema.graphql"
)]
#[allow(dead_code)]
struct InterfaceQuery;

#[test]
fn interface_deserialization() {
    println!("{:?}", RESPONSE);
    let response_data: interface_query::ResponseData = serde_json::from_str(RESPONSE).unwrap();

    println!("{:?}", response_data);

    let expected = r##"ResponseData { everything: Some([RustMyQueryEverything { name: "Audrey Lorde", on: Person(RustMyQueryEverythingOnPerson { birthday: Some("1934-02-18") }) }, RustMyQueryEverything { name: "Laïka", on: Dog(RustMyQueryEverythingOnDog { isGoodDog: true }) }, RustMyQueryEverything { name: "Mozilla", on: Organization(RustMyQueryEverythingOnOrganization { industry: OTHER }) }, RustMyQueryEverything { name: "Norbert", on: Dog(RustMyQueryEverythingOnDog { isGoodDog: true }) }]) }"##;

    assert_eq!(format!("{:?}", response_data), expected);

    assert_eq!(response_data.everything.map(|names| names.len()), Some(4));
}
