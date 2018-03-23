extern crate jenkins_api;

use jenkins_api::JenkinsBuilder;

#[test]
fn can_get_jenkins_home() {
    let jenkins = JenkinsBuilder::new("http://localhost:8080")
        .with_user("user", Some("password"))
        .build()
        .unwrap();
    assert!(jenkins.get_home().is_ok());
}

#[test]
fn should_be_forbidden() {
    let jenkins = JenkinsBuilder::new("http://localhost:8080")
        .with_user("unknown", Some("password"))
        .build()
        .unwrap();
    let response = jenkins.get_home();
    assert!(response.is_err());
    assert_eq!(
        format!("{:?}", response),
        "Err(Error { kind: ClientError(Unauthorized), url: Some(\"http://localhost:8080/api/json\") })"
    );
}

#[test]
fn should_be_connection_error() {
    let jenkins = JenkinsBuilder::new("http://localhost:808").build().unwrap();
    let response = jenkins.get_home();
    assert!(response.is_err());
}

#[test]
fn can_get_view() {
    let jenkins = JenkinsBuilder::new("http://localhost:8080")
        .with_user("user", Some("password"))
        .build()
        .unwrap();
    assert!(jenkins.get_view("view disabled").is_ok());
}

#[test]
fn should_get_view_not_found() {
    let jenkins = JenkinsBuilder::new("http://localhost:8080")
        .with_user("user", Some("password"))
        .build()
        .unwrap();
    let response = jenkins.get_view("zut");
    assert!(response.is_err());
    assert_eq!(
        format!("{:?}", response),
        "Err(Error { kind: ClientError(NotFound), url: Some(\"http://localhost:8080/view/zut/api/json\") })"
    );
}

#[test]
fn can_get_job() {
    let jenkins = JenkinsBuilder::new("http://localhost:8080/")
        .with_user("user", Some("password"))
        .build()
        .unwrap();
    assert!(jenkins.get_job("normal job").is_ok());
}
