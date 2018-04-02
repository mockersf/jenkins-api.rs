extern crate jenkins_api;

use jenkins_api::JenkinsBuilder;
use std::{thread, time};

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
    let home_ok = home.unwrap();
    let first_view = home_ok
        .views
        .iter()
        .filter(|view| view.name == "view disabled")
        .nth(0)
        .unwrap();
    let full_view = first_view.get_full_view(&jenkins);
    assert!(full_view.is_ok());
    let full_job = full_view.unwrap().jobs[0].get_full_job(&jenkins);
    assert!(full_job.is_ok());
}

#[test]
fn can_get_build_from_job_and_back() {
    let jenkins = JenkinsBuilder::new("http://localhost:8080/")
        .with_user("user", Some("password"))
        .build()
        .unwrap();
    let job = jenkins.get_job("normal job");
    assert!(job.is_ok());
    let job_ok = job.unwrap();
    let build = job_ok.last_build.unwrap().get_full_build(&jenkins);
    assert!(build.is_ok());
    let job_back = build.unwrap().get_job(&jenkins);
    assert!(job_back.is_ok());
    assert_eq!(job_back.unwrap().name, job_ok.name);
}

#[test]
fn can_disable_job_and_reenable() {
    let jenkins = JenkinsBuilder::new("http://localhost:8080/")
        .with_user("user", Some("password"))
        .build()
        .unwrap();
    let job = jenkins.get_job("normal job");
    assert!(job.is_ok());
    let job_ok = job.unwrap();
    assert!(job_ok.buildable);

    let disabling = job_ok.disable(&jenkins);
    assert!(disabling.is_ok());
    let job_disabled = jenkins.get_job("normal job");
    assert!(job_disabled.is_ok());
    let job_disabled_ok = job_disabled.unwrap();
    assert!(!job_disabled_ok.buildable);

    let enabling = job_disabled_ok.enable(&jenkins);
    assert!(enabling.is_ok());
    let job_enabled = jenkins.get_job("normal job");
    assert!(job_enabled.is_ok());
    let job_enabled_ok = job_enabled.unwrap();
    assert!(job_enabled_ok.buildable);
}

#[test]
fn can_add_and_remove_job_from_view() {
    let jenkins = JenkinsBuilder::new("http://localhost:8080/")
        .with_user("user", Some("password"))
        .build()
        .unwrap();

    let view = jenkins.get_view("test view");
    assert!(view.is_ok());
    let view_ok = view.unwrap();
    assert_eq!(view_ok.jobs.len(), 0);

    let job = jenkins.get_job("normal job");
    assert!(job.is_ok());
    let job_ok = job.unwrap();

    let adding = view_ok.add_job(&jenkins, &job_ok.name);
    assert!(adding.is_ok());

    let view_with = jenkins.get_view("test view");
    assert!(view_with.is_ok());
    assert_eq!(view_with.unwrap().jobs.len(), 1);

    let removing = job_ok.remove_from_view(&jenkins, &view_ok.name);
    assert!(removing.is_ok());

    let view_without = jenkins.get_view("test view");
    assert!(view_without.is_ok());
    assert_eq!(view_without.unwrap().jobs.len(), 0);
}

#[test]
fn can_get_queue() {
    let jenkins = JenkinsBuilder::new("http://localhost:8080/")
        .with_user("user", Some("password"))
        .build()
        .unwrap();
    let job = jenkins.get_job("long job");
    assert!(job.is_ok());
    let job_ok = job.unwrap();
    let triggered = job_ok.build(&jenkins);
    assert!(triggered.is_ok());
    let queue = jenkins.get_queue();
    assert!(queue.is_ok());
}

#[test]
fn can_get_queue_item() {
    let jenkins = JenkinsBuilder::new("http://localhost:8080/")
        .with_user("user", Some("password"))
        .build()
        .unwrap();

    let job = jenkins.get_job("job name");
    assert!(job.is_ok());
    let triggered = job.unwrap().build(&jenkins);
    assert!(triggered.is_ok());

    let triggered_ok = triggered.unwrap();

    let few_seconds = time::Duration::from_secs(2);
    for _ in 0..5 {
        assert!(triggered_ok.get_full_queue_item(&jenkins).is_ok());
        thread::sleep(few_seconds);
    }
}
