# jenkins-api.rs [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT) [![Build Status](https://travis-ci.org/mockersf/jenkins-api.rs.svg?branch=master)](https://travis-ci.org/mockersf/jenkins-api.rs)

Bindings to [Jenkins JSON API](https://wiki.jenkins.io/display/JENKINS/Remote+access+API)

## Example

```rust
extern crate jenkins_api;

fn main() {
    let jenkins = JenkinsBuilder::new("http://localhost:8080")
        .with_user("user", Some("password"))
        .build()
        .unwrap();

    let job = jenkins.get_job("job name").unwrap();
    let build = job.last_build.unwrap().get_full_build(&jenkins).unwrap();

    println!(
        "last build for job {} at {} was {:?}",
        job.name, build.timestamp, build.result
    );
}
```
