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
    let job = jenkins.get_job("normal job");
    assert!(job.is_ok());
    assert!(
        job.unwrap()
            .last_build
            .unwrap()
            .get_full_build(&jenkins)
            .is_ok()
    );
}

#[test]
fn can_get_build() {
    let jenkins = JenkinsBuilder::new("http://localhost:8080")
        .with_user("user", Some("password"))
        .build()
        .unwrap();
    assert!(jenkins.get_build("normal job", 1).is_ok());
}

#[test]
fn can_get_jenkins_view_from_home() {
    let jenkins = JenkinsBuilder::new("http://localhost:8080")
        .with_user("user", Some("password"))
        .build()
        .unwrap();
    let home = jenkins.get_home();
    assert!(home.is_ok());
    let first_view = &home.unwrap().views[1];
    let full_view = first_view.get_full_view(&jenkins);
    assert!(full_view.is_ok());
    let full_job = full_view.unwrap().jobs[0].get_full_job(&jenkins);
    assert!(full_job.is_ok());
}
