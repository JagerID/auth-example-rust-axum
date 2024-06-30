use super::{dto::FilteredProject, model::Project};

pub fn filter_project(project: &Project) -> FilteredProject {
    FilteredProject {
        id: project.id.to_owned(),
        name: project.name.to_owned(),
        created_at: project.created_at.to_owned(),
        updated_at: project.updated_at.to_owned(),
        is_public: project.is_public,
    }
}

pub fn filter_projects(projects: &Vec<Project>) -> Vec<FilteredProject> {
    projects.iter().map(filter_project).collect()
}
