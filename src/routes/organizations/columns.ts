import type { Organization } from "$lib/bindings";
import { tableFeatures, type ColumnDef } from "@tanstack/svelte-table";

export const features = tableFeatures({})
export const columns: ColumnDef<typeof features, Organization>[] = [
  { accessorKey: "name", header: "Name" },
  { accessorFn: (row) => row.positions.length, id: "positions", header: "Positions" },
  { accessorFn: (row) => row.employees.length, id: "employees", header: "Employees" },
  { accessorFn: (row) => row.shifts.length, id: "shifts", header: "Shifts" },
  { accessorFn: (row) => row.roster_templates.length, id: "roster_templates", header: "Roster Templates" },
  { accessorFn: (row) => row.rosters.length, id: "rosters", header: "Rosters" },
]
