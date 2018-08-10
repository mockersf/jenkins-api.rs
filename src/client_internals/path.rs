use urlencoding;

use super::Jenkins;
use build;

/// Name of an object
#[derive(Debug, PartialEq)]
pub enum Name<'a> {
    /// Name of an object
    Name(&'a str),
    /// URL Encoded name of an object
    UrlEncodedName(&'a str),
}

impl<'a> ToString for Name<'a> {
    fn to_string(&self) -> String {
        match *self {
            Name::Name(name) => urlencoding::encode(name),
            Name::UrlEncodedName(name) => name.to_string(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Path<'a> {
    Home,
    View {
        name: Name<'a>,
    },
    AddJobToView {
        job_name: Name<'a>,
        view_name: Name<'a>,
    },
    RemoveJobFromView {
        job_name: Name<'a>,
        view_name: Name<'a>,
    },
    Job {
        name: Name<'a>,
        configuration: Option<Name<'a>>,
    },
    BuildJob {
        name: Name<'a>,
    },
    BuildJobWithParameters {
        name: Name<'a>,
    },
    PollSCMJob {
        name: Name<'a>,
    },
    JobEnable {
        name: Name<'a>,
    },
    JobDisable {
        name: Name<'a>,
    },
    Build {
        job_name: Name<'a>,
        number: build::BuildNumber,
        configuration: Option<Name<'a>>,
    },
    ConsoleText {
        job_name: Name<'a>,
        number: build::BuildNumber,
        configuration: Option<Name<'a>>,
    },
    Queue,
    QueueItem {
        id: i32,
    },
    MavenArtifactRecord {
        job_name: Name<'a>,
        number: build::BuildNumber,
        configuration: Option<Name<'a>>,
    },
    Computers,
    Computer {
        name: Name<'a>,
    },
    Raw {
        path: &'a str,
    },
    CrumbIssuer,
}

impl<'a> ToString for Path<'a> {
    fn to_string(&self) -> String {
        match *self {
            Path::Home => "".to_string(),
            Path::View { ref name } => format!("/view/{}", name.to_string()),
            Path::AddJobToView {
                ref job_name,
                ref view_name,
            } => format!(
                "/view/{}/addJobToView?name={}",
                view_name.to_string(),
                job_name.to_string()
            ),
            Path::RemoveJobFromView {
                ref job_name,
                ref view_name,
            } => format!(
                "/view/{}/removeJobFromView?name={}",
                view_name.to_string(),
                job_name.to_string()
            ),
            Path::Job {
                ref name,
                configuration: Some(ref configuration),
            } => format!("/job/{}/{}", name.to_string(), configuration.to_string()),
            Path::Job {
                ref name,
                configuration: None,
            } => format!("/job/{}", name.to_string()),
            Path::BuildJob { ref name } => format!("/job/{}/build", name.to_string()),
            Path::BuildJobWithParameters { ref name } => {
                format!("/job/{}/buildWithParameters", name.to_string())
            }
            Path::PollSCMJob { ref name } => format!("/job/{}/polling", name.to_string()),
            Path::JobEnable { ref name } => format!("/job/{}/enable", name.to_string()),
            Path::JobDisable { ref name } => format!("/job/{}/disable", name.to_string()),
            Path::Build {
                ref job_name,
                ref number,
                configuration: None,
            } => format!("/job/{}/{}", job_name.to_string(), number.to_string()),
            Path::Build {
                ref job_name,
                ref number,
                configuration: Some(ref configuration),
            } => format!(
                "/job/{}/{}/{}",
                job_name.to_string(),
                configuration.to_string(),
                number.to_string()
            ),
            Path::ConsoleText {
                ref job_name,
                ref number,
                configuration: None,
            } => format!(
                "/job/{}/{}/consoleText",
                job_name.to_string(),
                number.to_string()
            ),
            Path::ConsoleText {
                ref job_name,
                ref number,
                configuration: Some(ref configuration),
            } => format!(
                "/job/{}/{}/{}/consoleText",
                job_name.to_string(),
                configuration.to_string(),
                number.to_string()
            ),
            Path::Queue => "/queue".to_string(),
            Path::QueueItem { ref id } => format!("/queue/item/{}", id),
            Path::MavenArtifactRecord {
                ref job_name,
                ref number,
                configuration: None,
            } => format!(
                "/job/{}/{}/mavenArtifacts",
                job_name.to_string(),
                number.to_string()
            ),
            Path::MavenArtifactRecord {
                ref job_name,
                ref number,
                configuration: Some(ref configuration),
            } => format!(
                "/job/{}/{}/{}/mavenArtifacts",
                job_name.to_string(),
                configuration.to_string(),
                number.to_string()
            ),
            Path::Computers => "/computer/api/json".to_string(),
            Path::Computer { ref name } => format!("/computer/{}/api/json", name.to_string()),
            Path::Raw { path } => path.to_string(),
            Path::CrumbIssuer => "/crumbIssuer".to_string(),
        }
    }
}

impl Jenkins {
    pub(crate) fn url_to_path<'a>(&self, url: &'a str) -> Path<'a> {
        let path = if url.starts_with(&self.url) {
            &url[self.url.len()..]
        } else {
            url
        };
        let slashes: Vec<usize> = path
            .char_indices()
            .filter(|c| c.1 == '/')
            .map(|c| c.0)
            .collect();

        match (&path[0..slashes[1]], slashes.len()) {
            ("/view", 3) => Path::View {
                name: Name::UrlEncodedName(&path[6..(path.len() - 1)]),
            },
            ("/job", 3) => Path::Job {
                name: Name::UrlEncodedName(&path[5..(path.len() - 1)]),
                configuration: None,
            },
            ("/job", 4) => {
                let last_part = &path[(slashes[2] + 1)..(path.len() - 1)];
                let number = last_part.parse();
                if let Ok(number) = number {
                    Path::Build {
                        job_name: Name::UrlEncodedName(&path[5..slashes[2]]),
                        number: build::BuildNumber::Number(number),
                        configuration: None,
                    }
                } else {
                    Path::Job {
                        name: Name::UrlEncodedName(&path[5..slashes[2]]),
                        configuration: Some(Name::UrlEncodedName(last_part)),
                    }
                }
            }
            ("/job", 5) => {
                if &path[slashes[3]..slashes[4]] == "mavenArtifacts" {
                    Path::MavenArtifactRecord {
                        job_name: Name::UrlEncodedName(&path[5..slashes[2]]),
                        number: build::BuildNumber::Number(
                            path[(slashes[3] + 1)..(path.len() - 1)].parse().unwrap(),
                        ),
                        configuration: None,
                    }
                } else {
                    Path::Build {
                        job_name: Name::UrlEncodedName(&path[5..slashes[2]]),
                        number: build::BuildNumber::Number(
                            path[(slashes[3] + 1)..(path.len() - 1)].parse().unwrap(),
                        ),
                        configuration: Some(Name::UrlEncodedName(
                            &path[(slashes[2] + 1)..slashes[3]],
                        )),
                    }
                }
            }
            ("/job", 6) => Path::MavenArtifactRecord {
                job_name: Name::UrlEncodedName(&path[5..slashes[2]]),
                number: build::BuildNumber::Number(
                    path[(slashes[3] + 1)..slashes[4]].parse().unwrap(),
                ),
                configuration: Some(Name::UrlEncodedName(&path[(slashes[2] + 1)..slashes[3]])),
            },
            ("/queue", 4) => Path::QueueItem {
                id: path[(slashes[2] + 1)..(path.len() - 1)].parse().unwrap(),
            },
            (_, _) => Path::Raw { path },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static JENKINS_URL: &'static str = "http://none:8080";

    #[test]
    fn can_parse_view_path() {
        let jenkins_client = ::JenkinsBuilder::new(JENKINS_URL).build().unwrap();

        let path = jenkins_client.url_to_path("/view/myview/");
        assert_eq!(
            path,
            Path::View {
                name: Name::UrlEncodedName("myview")
            }
        );
    }

    #[test]
    fn can_parse_job_path() {
        let jenkins_client = ::JenkinsBuilder::new(JENKINS_URL).build().unwrap();

        let path = jenkins_client.url_to_path("/job/myjob/");
        assert_eq!(
            path,
            Path::Job {
                name: Name::UrlEncodedName("myjob"),
                configuration: None
            }
        );
    }

    #[test]
    fn can_parse_job_with_config_path() {
        let jenkins_client = ::JenkinsBuilder::new(JENKINS_URL).build().unwrap();

        let path = jenkins_client.url_to_path("/job/myjob/config/");
        assert_eq!(
            path,
            Path::Job {
                name: Name::UrlEncodedName("myjob"),
                configuration: Some(Name::UrlEncodedName("config"))
            }
        );
    }

    #[test]
    fn can_parse_build_path() {
        let jenkins_client = ::JenkinsBuilder::new(JENKINS_URL).build().unwrap();

        let path = jenkins_client.url_to_path("/job/myjob/1/");
        assert_eq!(
            path,
            Path::Build {
                job_name: Name::UrlEncodedName("myjob"),
                number: build::BuildNumber::Number(1),
                configuration: None
            }
        );
    }

    #[test]
    fn can_parse_build_with_config_path() {
        let jenkins_client = ::JenkinsBuilder::new(JENKINS_URL).build().unwrap();

        let path = jenkins_client.url_to_path("/job/myjob/config/1/");
        assert_eq!(
            path,
            Path::Build {
                job_name: Name::UrlEncodedName("myjob"),
                number: build::BuildNumber::Number(1),
                configuration: Some(Name::UrlEncodedName("config"))
            }
        );
    }

    #[test]
    fn can_parse_unknown_path() {
        let jenkins_client = ::JenkinsBuilder::new(JENKINS_URL).build().unwrap();

        let path = jenkins_client.url_to_path("/unknown/path/");
        assert_eq!(
            path,
            Path::Raw {
                path: "/unknown/path/"
            }
        );
    }

    #[test]
    fn can_parse_job_path_with_jenkins_url() {
        let jenkins_client = ::JenkinsBuilder::new(JENKINS_URL).build().unwrap();

        let path_url = format!("{}/job/myjob/", JENKINS_URL);
        let path = jenkins_client.url_to_path(&path_url);
        assert_eq!(
            path,
            Path::Job {
                name: Name::UrlEncodedName("myjob"),
                configuration: None
            }
        );
    }
}
