#![allow(dead_code)]

use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use uuid::Uuid;

#[derive(Debug, serde::Serialize, serde::Deserialize, specta::Type)]
pub struct Project {
    pub organizations: Vec<Organization>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, specta::Type)]
pub struct Organization {
    pub id: Uuid,
    pub name: String,
    pub positions: Vec<Position>,
    pub employees: Vec<Employee>,
    pub shifts: Vec<Shift>,
    pub roster_templates: Vec<RosterTemplate>,
    pub rosters: Vec<Roster>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, specta::Type)]
pub struct Position {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, specta::Type)]
pub struct Employee {
    pub id: Uuid,
    pub name: String,
    pub position_id: Uuid,
    // TODO
    // unavailabilities
    // assignments
}

#[derive(Debug, serde::Serialize, serde::Deserialize, specta::Type)]
pub struct Shift {
    pub id: Uuid,
    pub name: String,
    pub start: NaiveTime,
    pub end: NaiveTime,
    // TODO
    // staffing_requirements
    // weekdays
}

#[derive(Debug, serde::Serialize, serde::Deserialize, specta::Type)]
pub struct RosterTemplate {
    pub id: Uuid,
    pub name: String,
    pub shifts: Vec<Shift>,
    // TODO
    // approved_position_ids
    // excluded_employee_id
    // rest_rules
    // sharing_groups
}

#[derive(Debug, serde::Serialize, serde::Deserialize, specta::Type)]
pub struct Roster {
    pub id: Uuid,
    pub name: String,
    pub template_id: Uuid,
    pub start: NaiveDate,
    pub end: NaiveDate,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    // TODO
    // plan_status
    // context_assignments
}
