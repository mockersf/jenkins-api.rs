use urlencoding;

use super::Jenkins;

#[derive(Debug)]
pub(crate) enum Name<'a> {
    Name(&'a str),
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

#[derive(Debug)]
pub(crate) enum Path<'a> {
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
    },
    BuildJob {
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
        number: u32,
    },
    ConsoleText {
        job_name: Name<'a>,
        number: u32,
    },
    Queue,
    QueueItem {
        id: u32,
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
            Path::Job { ref name } => format!("/job/{}", name.to_string()),
            Path::BuildJob { ref name } => format!("/job/{}/build", name.to_string()),
            Path::JobEnable { ref name } => format!("/job/{}/enable", name.to_string()),
            Path::JobDisable { ref name } => format!("/job/{}/disable", name.to_string()),
            Path::Build {
                ref job_name,
                ref number,
            } => format!("/job/{}/{}", job_name.to_string(), number),
            Path::ConsoleText {
                ref job_name,
                ref number,
            } => format!("/job/{}/{}/consoleText", job_name.to_string(), number),
            Path::Queue => "/queue".to_string(),
            Path::QueueItem { ref id } => format!("/queue/item/{}", id),
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
        let first_slash = path.char_indices().filter(|c| c.1 == '/').nth(1).unwrap().0;
        match (
            &path[0..first_slash],
            path.chars().filter(|c| *c == '/').count(),
        ) {
            ("/view", 3) => Path::View {
                name: Name::UrlEncodedName(&path[6..(path.len() - 1)]),
            },
            ("/job", 3) => Path::Job {
                name: Name::UrlEncodedName(&path[5..(path.len() - 1)]),
            },
            ("/job", 4) => {
                let last_slash = path.char_indices()
                    .rev()
                    .filter(|c| c.1 == '/')
                    .nth(1)
                    .unwrap()
                    .0;
                Path::Build {
                    job_name: Name::UrlEncodedName(&path[5..last_slash]),
                    number: path[(last_slash + 1)..(path.len() - 1)].parse().unwrap(),
                }
            }
            ("/queue", 4) => {
                let last_slash = path.char_indices()
                    .rev()
                    .filter(|c| c.1 == '/')
                    .nth(1)
                    .unwrap()
                    .0;
                Path::QueueItem {
                    id: path[(last_slash + 1)..(path.len() - 1)].parse().unwrap(),
                }
            }
            (_, _) => Path::Raw { path },
        }
    }
}
