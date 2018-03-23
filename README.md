# jenkins-api.rs [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT) 

Bindings to [Jenkins JSON API](https://wiki.jenkins.io/display/JENKINS/Remote+access+API)

## Example

```rust
let jenkins = JenkinsBuilder::new("http://localhost:8080".to_owned())
    .with_user("user".to_owned(), Some("password".to_owned()))
    .build()
    .unwrap();
let job = jenkins.get_job("job name")unwrap();
```

