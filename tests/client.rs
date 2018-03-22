extern crate jenkins_api;

use jenkins_api::JenkinsBuilder;

#[test]
fn can_get_jenkins_home() {
    let jenkins = JenkinsBuilder::new("http://localhost:8080".to_owned())
        .with_user("user".to_owned(), Some("password".to_owned()))
        .build()
        .unwrap();
    println!("{:?}", jenkins.get_home());
    assert!(jenkins.get_home().is_ok());
}

#[test]
fn should_be_forbidden() {
    let jenkins = JenkinsBuilder::new("http://localhost:8080".to_owned())
        .with_user("unknown".to_owned(), Some("password".to_owned()))
        .build()
        .unwrap();
    let response = jenkins.get_home();
    assert!(response.is_err());
    assert_eq!(
        format!("{:?}", response),
        "Err(Error { kind: ClientError(Unauthorized), url: Some(\"http://localhost:8080//api/json\") })"
    );
}

#[test]
fn should_be_connection_error() {
    let jenkins = JenkinsBuilder::new("http://localhost:808".to_owned())
        .build()
        .unwrap();
    let response = jenkins.get_home();
    assert!(response.is_err());
}

#[test]
fn can_get_view() {
    let jenkins = JenkinsBuilder::new("http://localhost:8080".to_owned())
        .with_user("user".to_owned(), Some("password".to_owned()))
        .build()
        .unwrap();
    println!("{:?}", jenkins.get_view("view%20disabled"));
    assert!(jenkins.get_view("view%20disabled").is_ok());
}

#[test]
fn should_get_view_not_found() {
    let jenkins = JenkinsBuilder::new("http://localhost:8080".to_owned())
        .with_user("user".to_owned(), Some("password".to_owned()))
        .build()
        .unwrap();
    let response = jenkins.get_view("zut");
    assert!(response.is_err());
    assert_eq!(
        format!("{:?}", response),
        "Err(Error { kind: ClientError(NotFound), url: Some(\"http://localhost:8080/view/zut/api/json\") })"
    );
}
