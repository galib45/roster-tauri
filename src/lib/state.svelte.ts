
import { commands, type Project } from "$lib/bindings";
import { createContext } from "svelte";

export class AppState {
  loading = $state(false);
  error = $state<string | null>(null);

  project = $state<Project | null>(null);
  hasOrganization = $derived((this.project?.organizations?.length ?? 0) > 0);
  selectedOrganizationId = $state<string | null>(null);
  selectedOrganization = $derived(
    this.project?.organizations?.find(
      org => org.id === this.selectedOrganizationId
    )
  );

  constructor() { }

  async load() {
    this.loading = true;
    this.error = null;

    try {
      const loadedProject = await commands.loadProject();
      this.project = loadedProject;

      if (!this.selectedOrganization) {
        this.selectedOrganizationId = loadedProject.organizations?.[0]?.id ?? null;
      }
    } catch (e) {
      this.error = String(e);
    } finally {
      this.loading = false;
    }
  }

  get organizations() {
    return this.project?.organizations ?? [];
  }

  selectOrganization(id: string) {
    if (this.project?.organizations?.some(org => org.id === id)) {
      this.selectedOrganizationId = id;
    }
  }
}

// export function getAppState() {
//   // Project data
//   let project = $state<Project | null>(null);
//
//   // Loading and error states
//   let loading = $state(false);
//   let error = $state<string | null>(null);
//
//   // Organization selection
//   let selectedOrganizationId = $state("");
//
//   // Derived state
//   let selectedOrganization = $derived(
//     project?.organizations?.find(
//       org => org.id === selectedOrganizationId
//     )
//   );
//
//   let hasOrganization = $derived(
//     (project?.organizations?.length ?? 0) > 0
//   );
//
//   // Load project from Rust
//   async function load() {
//     loading = true;
//     error = null;
//
//     try {
//       const loadedProject = await commands.loadProject();
//
//       project = loadedProject;
//
//       // Preserve selection if the organization still exists.
//       const organizationExists = loadedProject.organizations?.some(
//         org => org.id === selectedOrganizationId
//       );
//
//       if (!organizationExists) {
//         selectedOrganizationId =
//           loadedProject.organizations?.[0]?.id ?? "";
//       }
//     } catch (e) {
//       error = String(e);
//     } finally {
//       loading = false;
//     }
//   }
//
//   // Select organization
//   function selectOrganization(id: string) {
//     if (project?.organizations?.some(org => org.id === id)) {
//       selectedOrganizationId = id;
//     }
//   }
//
//   return {
//     // Project data
//     get project() {
//       return project;
//     },
//
//     get organizations() {
//       return project?.organizations ?? [];
//     },
//
//     get selectedOrganization() {
//       return selectedOrganization;
//     },
//
//     get selectedOrganizationId() {
//       return selectedOrganizationId;
//     },
//
//     // Loading and error
//     get loading() {
//       return loading;
//     },
//
//     get error() {
//       return error;
//     },
//
// //     // UI state
// //     get inputText() {
// //       return inputText;
// //     },
// //
// //     set inputText(value: string) {
// //       inputText = value;
// //     },
//
//     // Derived state
//     get hasOrganization() {
//       return hasOrganization;
//     },
//
//     // Actions
//     load,
//     selectOrganization,
//   };
// }

// export type AppState = ReturnType<typeof getAppState>;

export const [getAppState, setAppState] = createContext<AppState>();
