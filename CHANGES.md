# Changes

# 0.3.1

* Get artifacts of a build
* Support external jobs
* Support maven projects

# 0.3.0 (2018/05/13)

* Default enum variant renamed to Unknown
* Changed `Error::InvalidUrl` `expected` field to an Enum (`error::ExpectedType`)
* A `Build` can have many variants, for now either a free style or a pipeline
* Adding `Action` and change set variants
* Support pipeline `Job`
* Support more types of `View`
* Support matrix projects

# 0.2.2 (2018/05/10)

* Can deserialize git informations from a build
* Can trigger job remotely (GET request with a token)
* Can poll configured SCM of a project
* Can build job with parameters
* Can deserialize actions from a queue item
* Logging request and error responses

# 0.2.1 (2018/05/04)

* Can deserialize actions from a build

# 0.2.0 (2018/04/25)

* Fix case for error messages
* Better Queue management
* Can trigger job without parameters
* Can get console text from a build
