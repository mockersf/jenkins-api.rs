# Changes

# 0.5.0 (2018/07/02)

* Updated variants types to be able to keep variant information when navigating between objects (ie `CommonJob` -> `FreeStyleProject` -> `ShortBuild` -> `FreeStyleBuild` without going through `CommonBuild`)
* Updated visibility of some of the structs in `client`
* Added a new method `get_object_as` that let the user decide the amount of data returned. See [taming-jenkins-json-api-depth-and-tree](https://www.cloudbees.com/blog/taming-jenkins-json-api-depth-and-tree)
* Removed deprecated methods

# 0.4.2 (2018/06/19)

* Decrease log level
* Deprecated most functions of traits `Job` and `Build`
* Add fields on TimeInQueueAction
* Support MultiJobProject and MultiJobBuild

# 0.4.1 (2018/06/13)

* Can get nodes linked to Jenkins
* Support build flow jobs

# 0.4.0 (2018/05/24)

* Change all data structures to extendable trait / struct instead of enum
* Can change depth in requests when building Jenkins client
* All short items derive Serialize
* Can target build by alias

# 0.3.1 (2018/05/21)

* Get artifacts of a build
* Support external jobs
* Support maven projects
* Feature to toggle between permissive/strict json parsing for Jenkins responses

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
