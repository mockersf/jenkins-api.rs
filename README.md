# jenkins-api.rs [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT) [![Build Status](https://travis-ci.org/mockersf/jenkins-api.rs.svg?branch=master)](https://travis-ci.org/mockersf/jenkins-api.rs) [![Coverage Status](https://coveralls.io/repos/github/mockersf/jenkins-api.rs/badge.svg?branch=master)](https://coveralls.io/github/mockersf/jenkins-api.rs?branch=master) [![Realease Doc](https://docs.rs/jenkins_api/badge.svg)](https://docs.rs/jenkins_api) [![Crate](https://img.shields.io/crates/v/jenkins_api.svg)](https://crates.io/crates/jenkins_api)

Bindings to [Jenkins JSON API](https://wiki.jenkins.io/display/JENKINS/Remote+access+API)

The API docs for the master branch are published [here](https://mockersf.github.io/jenkins-api.rs/).

## Example

```rust
extern crate failure;

extern crate jenkins_api;

use jenkins_api::{JenkinsBuilder, BuildStatus};

fn main() -> Result<(), failure::Error> {
    let jenkins = JenkinsBuilder::new("http://localhost:8080")
        .with_user("user", Some("password"))
        .build()?;

    let job = jenkins.get_job("job name")?;

    let to_build = if let Some(short_build) = job.last_build()?.clone() {
        let build = short_build.get_full_build(&jenkins)?;
        println!(
            "last build for job {} at {} was {:?}",
            job.name()?, build.timestamp()?, build.result()?
        );
        build.result()? != BuildStatus::Success
    } else {
        println!("job {} was never built", job.name()?);
        true
    };

    if to_build {
        println!("triggering a new build");
        job.build(&jenkins)?;
    }
    Ok(())
}
```
