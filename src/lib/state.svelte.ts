import { commands, type Project } from "$lib/bindings"
import { createContext } from "svelte";

export function getAppState() {
  let project = $state<Project | null>(null);
  let loading = $state(false);
  let error = $state<string | null>(null);

  async function load() {
    loading = true;
    error = null;
    try { project = await commands.loadProject(); }
    catch (e) { error = String(e); } finally { loading = false; }
  }

  return {
      get project() {
          return project;
      },
      get loading() {
          return loading;
      },
      get error() {
          return error;
      },
      load,
  };
}

export function hasOrganization(appState: AppState) {
  return (appState.project?.organizations?.length ?? 0) > 0
}

export type AppState = ReturnType<typeof getAppState>;
export const [getProjectContext, setProjectContext] = createContext<Project>();
